package com.musicfrog.despicableinfiltrator.ui.settings.dns

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.musicfrog.despicableinfiltrator.R
import com.musicfrog.despicableinfiltrator.ui.common.ErrorDialog
import com.musicfrog.despicableinfiltrator.ui.common.InputDialog
import com.musicfrog.despicableinfiltrator.ui.common.SelectionDialog
import com.musicfrog.despicableinfiltrator.ui.common.StandardListItem

@Composable
fun DnsScreen(viewModel: DnsViewModel = viewModel()) {
    val state by viewModel.state.collectAsState()
    var showModeDialog by remember { mutableStateOf(false) }
    var showNameserverDialog by remember { mutableStateOf(false) }
    var showDefaultDialog by remember { mutableStateOf(false) }
    var showFallbackDialog by remember { mutableStateOf(false) }

    if (showModeDialog) {
        SelectionDialog(
            title = stringResource(R.string.label_enhanced_mode),
            options = listOf(
                "" to "Disabled",
                "fake-ip" to "Fake-IP",
                "redir-host" to "Redir-Host"
            ),
            selectedOption = state.enhancedMode,
            onDismiss = { showModeDialog = false },
            onSelect = {
                viewModel.updateEnhancedMode(it)
                showModeDialog = false
            }
        )
    }

    if (showNameserverDialog) {
        InputDialog(
            title = stringResource(R.string.label_nameserver),
            initialValue = state.nameserver,
            onDismiss = { showNameserverDialog = false },
            onConfirm = {
                viewModel.updateNameserver(it)
                showNameserverDialog = false
            },
            singleLine = false
        )
    }

    if (showDefaultDialog) {
        InputDialog(
            title = stringResource(R.string.label_default_nameserver),
            initialValue = state.defaultNameserver,
            onDismiss = { showDefaultDialog = false },
            onConfirm = {
                viewModel.updateDefaultNameserver(it)
                showDefaultDialog = false
            },
            singleLine = false
        )
    }

    if (showFallbackDialog) {
        InputDialog(
            title = stringResource(R.string.label_fallback),
            initialValue = state.fallback,
            onDismiss = { showFallbackDialog = false },
            onConfirm = {
                viewModel.updateFallback(it)
                showFallbackDialog = false
            },
            singleLine = false
        )
    }

    Scaffold(
        bottomBar = {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                horizontalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(12.dp)
            ) {
                Button(
                    onClick = { viewModel.save() },
                    enabled = !state.isLoading,
                    modifier = Modifier.weight(1f)
                ) {
                    Text(stringResource(R.string.action_save))
                }
                TextButton(
                    onClick = { viewModel.load() },
                    enabled = !state.isLoading
                ) {
                    Text(stringResource(R.string.action_reload))
                }
            }
        }
    ) { padding ->
        Box(modifier = Modifier.padding(padding).fillMaxSize()) {
            if (state.isLoading) {
                CircularProgressIndicator(modifier = Modifier.align(Alignment.Center))
            }

            if (state.error != null) {
                ErrorDialog(
                    message = state.error ?: "",
                    onDismiss = { viewModel.clearError() }
                )
            }

            LazyColumn(modifier = Modifier.fillMaxSize()) {
                item {
                    StandardListItem(
                        headline = stringResource(R.string.dns_enable),
                        supporting = stringResource(R.string.dns_enable_desc),
                        trailingContent = {
                            Switch(
                                checked = state.enabled,
                                onCheckedChange = { viewModel.updateEnabled(it) },
                                enabled = !state.isLoading
                            )
                        },
                        onClick = { if (!state.isLoading) viewModel.updateEnabled(!state.enabled) }
                    )
                    HorizontalDivider()
                }

                item {
                    StandardListItem(
                        headline = stringResource(R.string.label_ipv6),
                        supporting = stringResource(R.string.tun_ipv6_desc),
                        trailingContent = {
                            Switch(
                                checked = state.ipv6,
                                onCheckedChange = { viewModel.updateIpv6(it) },
                                enabled = !state.isLoading
                            )
                        },
                        onClick = { if (!state.isLoading) viewModel.updateIpv6(!state.ipv6) }
                    )
                    HorizontalDivider()
                }

                item {
                    StandardListItem(
                        headline = stringResource(R.string.label_enhanced_mode),
                        supporting = state.enhancedMode.ifBlank { "Disabled" },
                        onClick = { if (!state.isLoading) showModeDialog = true }
                    )
                    HorizontalDivider()
                }

                item {
                    StandardListItem(
                        headline = stringResource(R.string.label_nameserver),
                        supporting = state.nameserver.replace("\n", ", "),
                        onClick = { if (!state.isLoading) showNameserverDialog = true }
                    )
                    HorizontalDivider()
                }

                item {
                    StandardListItem(
                        headline = stringResource(R.string.label_default_nameserver),
                        supporting = state.defaultNameserver.replace("\n", ", "),
                        onClick = { if (!state.isLoading) showDefaultDialog = true }
                    )
                    HorizontalDivider()
                }

                item {
                    StandardListItem(
                        headline = stringResource(R.string.label_fallback),
                        supporting = state.fallback.replace("\n", ", "),
                        onClick = { if (!state.isLoading) showFallbackDialog = true }
                    )
                    HorizontalDivider()
                }
                
                if (state.saved) {
                    item {
                        Box(modifier = Modifier.fillMaxWidth().padding(16.dp), contentAlignment = Alignment.Center) {
                            Text(
                                text = stringResource(R.string.text_saved),
                                color = MaterialTheme.colorScheme.primary,
                                style = MaterialTheme.typography.labelLarge
                            )
                        }
                    }
                }
            }
        }
    }
}
