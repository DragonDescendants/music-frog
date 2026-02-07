import { createI18n } from 'vue-i18n';
import enUS from '../locales/en-US.json';

export function createTestI18n() {
  return createI18n({
    legacy: false,
    locale: 'en',
    fallbackLocale: 'en',
    messages: {
      en: enUS,
      'en-US': enUS,
    },
  });
}
