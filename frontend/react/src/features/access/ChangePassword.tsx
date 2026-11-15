import { Alert, Button, Form, Input } from "antd";
import { useState } from "react";
import { api } from "../../api/client";
import type { Session } from "../../api/client";
import { errorText, useI18n, useLocalizedForm } from "../../i18n/I18n";
import type { MessageKey } from "../../i18n/messages";

type Fields = {
  newPassword: string;
  confirm: string;
};

export function ChangePassword({
  user,
  onDone,
}: {
  user: Session;
  onDone: () => Promise<void>;
}) {
  const { t } = useI18n();
  const [form] = Form.useForm<Fields>();
  useLocalizedForm(form);
  const [error, setError] = useState<unknown>(null);
  const [done, setDone] = useState(false);
  const [busy, setBusy] = useState(false);
  async function submit(values: Fields) {
    if (values.newPassword !== values.confirm) {
      setError(t("passwordChange.mismatch"));
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await api(
        "/auth/password",
        "POST",
        { newPassword: values.newPassword },
        user.csrf,
      );
      setDone(true);
      await onDone();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  const message = (key: MessageKey) => t(key);
  return (
    <main className="login-shell">
      <section className="login-form">
        <div className="login-card">
          <p className="eyebrow">{user.username}</p>
          <h2>{message("passwordChange.title")}</h2>
          <p className="muted">{message("passwordChange.description")}</p>
          {error != null && (
            <Alert
              type="error"
              showIcon
              description={
                typeof error === "string"
                  ? error
                  : errorText(error, t)
              }
            />
          )}
          {done && <Alert type="success" showIcon description={message("passwordChange.success")} />}
          <Form<Fields>
            form={form}
            layout="vertical"
            disabled={done}
            onFinish={(values) => void submit(values)}
            requiredMark={false}
          >
            <Form.Item
              name="newPassword"
              label={message("passwordChange.new")}
              rules={[{ required: true, min: 12 }]}
            >
              <Input.Password autoComplete="new-password" size="large" />
            </Form.Item>
            <Form.Item
              name="confirm"
              label={message("passwordChange.confirm")}
              rules={[{ required: true, min: 12 }]}
            >
              <Input.Password autoComplete="new-password" size="large" />
            </Form.Item>
            <Button type="primary" size="large" htmlType="submit" block loading={busy}>
              {message("passwordChange.submit")}
            </Button>
          </Form>
        </div>
      </section>
    </main>
  );
}
