import { Alert, Button, Form, Input } from "antd";
import { useState } from "react";
import { api, ApiError } from "../../api/client";
import {
  LanguageSwitcher,
  errorText,
  useI18n,
  useLocalizedForm,
} from "../../i18n/I18n";
export function Login({ onLogin }: { onLogin: () => Promise<void> }) {
  const { t } = useI18n();
  const [form] = Form.useForm();
  useLocalizedForm(form);
  const [error, setError] = useState<unknown>(null);
  const [busy, setBusy] = useState(false);
  async function submit(values: { username: string; password: string }) {
    setBusy(true);
    setError(null);
    try {
      await api("/auth/login", "POST", values);
      await onLogin();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  return (
    <main className="login-shell">
      <section className="login-story">
        <a className="brand" href="/">
          YOFC <span>GATEWAY OPERATIONS</span>
        </a>
        <div>
          <p className="eyebrow">{t("login.eyebrow")}</p>
          <h1>
            {t("login.heading1")}
            <br />
            {t("login.heading2")}
          </h1>
          <p className="story-copy">
            {t("login.story1")}
            <br />
            {t("login.story2")}
          </p>
        </div>
        <footer>{t("login.footer")}</footer>
      </section>
      <section className="login-form">
        <div className="login-language">
          <LanguageSwitcher />
        </div>
        <div className="login-card">
          <p className="eyebrow">{t("app.center")}</p>
          <h2>{t("login.title")}</h2>
          <p className="muted">{t("login.subtitle")}</p>
          {error != null && (
            <Alert
              title={
                error instanceof ApiError && error.code === "UNAUTHENTICATED"
                  ? t("login.invalidCredentials")
                  : errorText(error, t)
              }
              type="error"
              showIcon
            />
          )}
          <Form
            form={form}
            layout="vertical"
            onFinish={submit}
            requiredMark={false}
          >
            <Form.Item
              label={t("login.username")}
              name="username"
              rules={[{ required: true, message: t("login.usernameRequired") }]}
            >
              <Input autoComplete="username" size="large" />
            </Form.Item>
            <Form.Item
              label={t("login.password")}
              name="password"
              rules={[{ required: true, message: t("login.passwordRequired") }]}
            >
              <Input.Password autoComplete="current-password" size="large" />
            </Form.Item>
            <Button
              type="primary"
              size="large"
              htmlType="submit"
              block
              loading={busy}
            >
              {t("login.submit")}
            </Button>
          </Form>
          <p className="login-note">{t("login.note")}</p>
        </div>
      </section>
    </main>
  );
}
