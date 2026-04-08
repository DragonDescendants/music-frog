use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, checkbox, column, container, pick_list, row, text};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("nav_settings")).size(24).font(bold_font);

    // 0. UAC Prompt (if not admin)
    let uac_banner = if !state.is_admin {
        Some(
            container(column![
                text(lang.tr("admin_status")).font(bold_font).size(16),
                Space::new().height(8),
                text(lang.tr("settings_uac_desc")).size(13),
                Space::new().height(15),
                button(text(lang.tr("settings_uac_request")).size(12))
                    .on_press(Message::RequestAdminPrivilege)
                    .padding([8, 16])
                    .style(button::primary),
            ])
            .padding(20)
            .width(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::from_rgba(0.8, 0.4, 0.0, 0.1).into()),
                border: Border {
                    radius: border::Radius::from(12.0),
                    width: 1.0,
                    color: Color::from_rgba(0.8, 0.4, 0.0, 0.5),
                },
                ..Default::default()
            }),
        )
    } else {
        None
    };

    // 1. System Integration Card
    let system_section = card(column![
        text(lang.tr("settings_system_integration")).font(bold_font),
        Space::new().height(15),
        checkbox(state.autostart_enabled)
            .label(lang.tr("autostart").into_owned())
            .on_toggle(Message::SetAutostart),
        Space::new().height(10),
        checkbox(state.system_proxy_enabled)
            .label(lang.tr("system_proxy").into_owned())
            .on_toggle(Message::SetSystemProxy),
        Space::new().height(10),
        checkbox(state.theme == Theme::Dark)
            .label(lang.tr("dark_mode").into_owned())
            .on_toggle(|_| Message::ToggleTheme),
        Space::new().height(20),
        button(text(lang.tr("settings_factory_reset").into_owned()).size(12))
            .on_press(Message::FactoryReset)
            .padding([8, 16])
            .style(button::danger),
    ]);

    // 2. Advanced TUN Settings
    let tun_section = card(column![
        row![
            text(lang.tr("tun_mode")).font(bold_font).width(Length::Fill),
            checkbox(state.tun_enabled.unwrap_or(false))
                .label(lang.tr("use").into_owned())
                .on_toggle(Message::SetTunEnabled),
        ]
        .align_y(Alignment::Center),
        Space::new().height(15),
        row![
            text(lang.tr("tun_stack")).size(14).width(Length::FillPortion(1)),
            pick_list(
                &["gvisor", "mixed", "system"][..],
                Some(state.tun_stack.as_str()),
                |s| Message::SetTunStack(s.to_string())
            )
            .width(Length::FillPortion(2)),
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        checkbox(state.tun_auto_route)
            .label(lang.tr("tun_auto_route").into_owned())
            .on_toggle(Message::SetTunAutoRoute),
        Space::new().height(10),
        checkbox(state.tun_strict_route)
            .label(lang.tr("tun_strict_route").into_owned())
            .on_toggle(Message::SetTunStrictRoute),
    ]);

    // 3. Traffic Sniffer
    let sniffer_section = card(column![
        row![
            text(lang.tr("settings_sniffer")).font(bold_font).width(Length::Fill),
            checkbox(state.sniffer_enabled)
                .label(lang.tr("use").into_owned())
                .on_toggle(Message::SetSnifferEnabled),
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text(lang.tr("settings_sniffer_desc"))
            .size(12)
            .style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.5, 0.5, 0.5))
            }),
    ]);

    // 4. Kernel Management
    let mut kernel_list = column![
        row![
            text(lang.tr("settings_kernel_mgmt"))
                .font(bold_font)
                .width(Length::Fill),
            if state.is_checking_update {
                Element::from(button(text(lang.tr("settings_checking").into_owned()).size(11)).padding([6, 12]))
            } else {
                button(text(lang.tr("settings_check_update").into_owned()).size(11))
                    .on_press(Message::CheckCoreUpdate)
                    .padding([6, 12])
                    .style(button::secondary)
                    .into()
            }
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
    ]
    .spacing(10);

    if let Some(latest) = &state.latest_core_version {
        let is_installed = state.installed_kernels.iter().any(|k| k.version == *latest);
        kernel_list = kernel_list.push(card(
            row![
                column![
                    text(lang.tr("settings_latest_version").replace("{0}", latest)).font(bold_font),
                    text(if is_installed {
                        lang.tr("settings_installed")
                    } else {
                        lang.tr("settings_available")
                    })
                    .size(11),
                ]
                .width(Length::Fill),
                if !is_installed {
                    if state.download_progress > 0.0 {
                        Element::from(
                            column![
                                text(format!("{:.1}%", state.download_progress * 100.0)).size(10),
                                container(Space::new().width(Length::Fixed(100.0)).height(4))
                                    .style(move |_: &Theme| container::Style {
                                        background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                                        ..Default::default()
                                    })
                            ]
                            .spacing(4),
                        )
                    } else {
                        button(text(lang.tr("settings_download").into_owned()).size(11))
                            .on_press(Message::DownloadCore(latest.clone()))
                            .padding([6, 12])
                            .style(button::primary)
                            .into()
                    }
                } else {
                    text("").into()
                }
            ]
            .align_y(Alignment::Center),
        ));
    }

    if state.installed_kernels.is_empty() {
        kernel_list = kernel_list.push(text(lang.tr("settings_no_kernels")).size(14));
    } else {
        for kernel in &state.installed_kernels {
            let is_default = kernel.is_default;

            let mut action_area = row![].spacing(8).align_y(Alignment::Center);

            if !is_default {
                action_area = action_area.push(
                    button(text(lang.tr("settings_delete").into_owned()).size(11))
                        .on_press(Message::DeleteKernel(kernel.version.clone()))
                        .padding([6, 12])
                        .style(button::danger),
                );
            }

            if is_default {
                action_area = action_area.push(
                    container(text("DEFAULT").size(10).font(bold_font))
                        .padding([4, 8])
                        .style(|_theme| container::Style {
                            background: Some(Color::from_rgb(0.2, 0.5, 0.8).into()),
                            border: Border {
                                radius: border::Radius::from(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                );
            } else {
                action_area = action_area.push(
                    button(text(lang.tr("settings_set_default").into_owned()).size(11))
                        .on_press(Message::SetDefaultKernel(kernel.version.clone()))
                        .padding([6, 12])
                        .style(button::secondary),
                );
            }

            kernel_list = kernel_list.push(
                container(
                    row![
                        column![
                            text(&kernel.version).font(bold_font),
                            text(kernel.path.to_string_lossy().to_string()).size(11),
                        ]
                        .width(Length::Fill),
                        action_area
                    ]
                    .align_y(Alignment::Center),
                )
                .padding(10)
                .style(|_theme| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()),
                    border: Border {
                        radius: border::Radius::from(8.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        }
    }

    let mut content = column![header, Space::new().height(20)].spacing(10);

    if let Some(banner) = uac_banner {
        content = content.push(banner);
        content = content.push(Space::new().height(10));
    }

    content = content
        .push(system_section)
        .push(Space::new().height(10))
        .push(tun_section)
        .push(Space::new().height(10))
        .push(sniffer_section)
        .push(Space::new().height(10))
        .push(card(kernel_list))
        .push(Space::new().height(40));

    Scrollable::new(content).height(Length::Fill).into()
}
