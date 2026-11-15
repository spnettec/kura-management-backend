import React, { useCallback, useEffect, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { Alert, Button, Space, Spin } from "antd";
import {
  I18nProvider,
  LanguageSwitcher,
  errorText,
  useI18n,
} from "./i18n/I18n";
import { translatedCode } from "./i18n/core";
import {
  api,
  ApiError,
  type Gateway,
  type Session,
  type Task,
} from "./api/client";
import { Login } from "./features/access/Login";
import { InitialSetup } from "./features/access/InitialSetup";
import { Settings } from "./features/settings/Settings";
import { ChangePassword } from "./features/access/ChangePassword";
import { Gateways } from "./features/gateways/Gateways";
import { Commands } from "./features/commands/Commands";
import "./style.css";
function App() {
  const { t } = useI18n();
  const [user, setUser] = useState<Session | null>();
  const [gateways, setGateways] = useState<Gateway[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [error, setError] = useState<unknown>(null);
  const [stream, setStream] = useState<
    "connecting" | "online" | "reconnecting"
  >("connecting");
  const [offset, setOffset] = useState(0);
  const sessionGeneration = useRef(0);
  const refreshSequence = useRef(0);
  const clearSessionData = useCallback(() => {
    sessionGeneration.current++;
    setGateways([]);
    setTasks([]);
    setOffset(0);
    setError(null);
  }, []);
  const commands = location.pathname === "/commands";
  const settings = location.pathname === "/settings";
  const loadSession = useCallback(async () => {
    clearSessionData();
    const generation = sessionGeneration.current;
    try {
      const session = await api<Session>("/session");
      if (generation === sessionGeneration.current) setUser(session);
    } catch (e) {
      if (generation !== sessionGeneration.current) return;
      if (e instanceof ApiError && e.status === 401) {
        setUser(null);
      } else {
        setError(e);
        setUser(null);
      }
    }
  }, [clearSessionData]);
  const refresh = useCallback(async () => {
    const generation = sessionGeneration.current;
    const sequence = ++refreshSequence.current;
    try {
      const [g, t] = await Promise.all([
        api<{ items: Gateway[] }>(`/gateways?offset=${offset}`),
        api<{ items: Task[] }>(`/tasks?offset=${offset}`),
      ]);
      if (
        generation === sessionGeneration.current &&
        sequence === refreshSequence.current
      ) {
        setGateways(g.items);
        setTasks(t.items);
      }
    } catch (e) {
      if (
        generation !== sessionGeneration.current ||
        sequence !== refreshSequence.current
      )
        return;
      if (e instanceof ApiError && e.status === 401) {
        clearSessionData();
        setUser(null);
      } else setError(e);
    }
  }, [offset, clearSessionData]);
  useEffect(() => {
    void loadSession();
  }, [loadSession]);
  useEffect(() => {
    if (!user || user.mustChangePassword) return;
    void refresh();
    const source = new EventSource("/api/v1/events");
    let closed = false;
    let running = false;
    let dirty = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const schedule = () => {
      dirty = true;
      if (closed || running || timer) return;
      timer = setTimeout(async () => {
        timer = undefined;
        if (closed) return;
        dirty = false;
        running = true;
        await refresh();
        running = false;
        if (dirty) schedule();
      }, 100);
    };
    source.onopen = () => setStream("online");
    source.onerror = () => {
      setStream("reconnecting");
      void loadSession();
    };
    for (const event of ["gateway", "task", "resync"])
      source.addEventListener(event, schedule);
    return () => {
      closed = true;
      clearTimeout(timer);
      source.close();
    };
  }, [user?.id, refresh, loadSession]);
  async function command(id: string, operation: string) {
    if (!user) return;
    try {
      await api(
        `/gateways/${encodeURIComponent(id)}/tasks`,
        "POST",
        { operation, idempotencyKey: crypto.randomUUID() },
        user.csrf,
      );
      setError(null);
      await refresh();
    } catch (e) {
      setError(e);
    }
  }
  async function logout() {
    if (!user) return;
    try {
      await api("/auth/logout", "POST", undefined, user.csrf);
      clearSessionData();
      setUser(null);
    } catch (e) {
      setError(e);
    }
  }
  if (user === undefined)
    return (
      <div className="loading">
        <Spin size="large" aria-label={t("app.loading")} />
      </div>
    );
  if (!user) return <Login onLogin={loadSession} />;
  if (user.mustChangePassword)
    return (
      <ChangePassword
        user={user}
        onDone={loadSession}
      />
    );
  const online = gateways.filter((g) => g.online).length;
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <a className="brand" href="/gateways">
          YOFC<span>{t("app.center")}</span>
        </a>
        <p className="nav-caption">{t("app.workspace")}</p>
        <nav>
          <a className={!commands ? "active" : ""} href="/gateways">
            <span>◫</span>
            {t("app.gateways")}
          </a>
          <a className={commands ? "active" : ""} href="/commands">
            <span>≡</span>
            {t("app.tasks")}
          </a>
          {user.role === "admin" && (
            <a className={settings ? "active" : ""} href="/settings">
              <span>⚙</span>
              {t("app.settings")}
            </a>
          )}
        </nav>
        <div className="sidebar-bottom">
          <span className="connection-dot" />
          {t(`stream.${stream}`)}
          <small>{t("app.channel")}</small>
        </div>
      </aside>
      <div className="workspace">
        <header className="topbar">
          <span>
            {t("app.operations")} <span className="slash">/</span>{" "}
            {settings ? t("app.settings") : commands ? t("app.tasks") : t("app.gateways")}
          </span>
          <Space>
            <LanguageSwitcher />
            <span>{user.username}</span>
            <TagRole role={user.role} />
            <Button type="text" onClick={() => void logout()}>
              {t("app.logout")}
            </Button>
          </Space>
        </header>
        <main className="main-content">
          <div className="page-heading">
            <div>
              <p className="eyebrow">
                  {t(settings ? "page.settingsEyebrow" : commands ? "page.tasksEyebrow" : "page.gatewaysEyebrow")}
              </p>
              <h1>{settings ? t("app.settings") : commands ? t("app.tasks") : t("app.gateways")}</h1>
              <p className="muted">
                {settings ? t("settings.description") : commands ? t("tasks.description") : t("overview.description")}
              </p>
            </div>
            {!settings && <Button onClick={() => void refresh()}>{t("app.refresh")}</Button>}
          </div>
          {error != null && (
            <Alert
              title={errorText(error, t)}
              type="error"
              closable
              onClose={() => setError(null)}
            />
          )}
          <div className="metrics">
            <div>
              <span>{t("metrics.gateways")}</span>
              <strong>{gateways.length.toString().padStart(2, "0")}</strong>
              <small>{t("metrics.registered")}</small>
            </div>
            <div>
              <span>{t("metrics.online")}</span>
              <strong className="green">
                {online.toString().padStart(2, "0")}
              </strong>
              <small>{t("metrics.heartbeat")}</small>
            </div>
            <div>
              <span>{t("metrics.kura")}</span>
              <strong>
                {gateways
                  .filter((g) => g.online && g.kuraAvailable)
                  .length.toString()
                  .padStart(2, "0")}
              </strong>
              <small>{t("metrics.bridge")}</small>
            </div>
            <div>
              <span>{t("metrics.pending")}</span>
              <strong>
                {tasks
                    .filter((t) => ["QUEUED", "RUNNING", "UNKNOWN"].includes(t.state))
                  .length.toString()
                  .padStart(2, "0")}
              </strong>
              <small>{t("metrics.tasks")}</small>
            </div>
          </div>
          <section className="data-card">
            <div className="section-heading">
              <h2>{settings ? t("app.settings") : commands ? t("table.tasks") : t("table.gateways")}</h2>
              <span className="muted">{t(`stream.${stream}`)}</span>
            </div>
            {settings ? (
              <Settings user={user} />
            ) : commands ? (
              <Commands items={tasks} />
            ) : (
              <Gateways
                onRegistered={() => void refresh()}
                items={gateways}
                user={user}
                onCommand={(id, op) => void command(id, op)}
              />
            )}
            <div className="pagination">
              <span className="muted">
                {t("pagination.page", { page: offset / 100 + 1 })}
              </span>
              <Space>
                <Button
                  disabled={offset === 0}
                  onClick={() => setOffset(Math.max(0, offset - 100))}
                >
                  {t("pagination.previous")}
                </Button>
                <Button
                  disabled={(commands ? tasks : gateways).length < 100}
                  onClick={() => setOffset(offset + 100)}
                >
                  {t("pagination.next")}
                </Button>
              </Space>
            </div>
          </section>
          <footer className="workspace-footer">
            YOFC EDGE OPERATIONS<span>{t("app.footer")}</span>
          </footer>
        </main>
      </div>
    </div>
  );
}
function TagRole({ role }: { role: string }) {
  const { t } = useI18n();
  return <span className="role-label">{translatedCode(t, "role", role)}</span>;
}
createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <I18nProvider>
        {location.pathname === "/setup" ? <InitialSetup /> : <App />}
    </I18nProvider>
  </React.StrictMode>,
);
