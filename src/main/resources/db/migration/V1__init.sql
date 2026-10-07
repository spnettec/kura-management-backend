create table tenants (
  tenant_id varchar(64) primary key,
  name varchar(255) not null,
  status varchar(32) not null default 'active',
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table gateway_groups (
  group_id varchar(64) primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  name varchar(255) not null,
  description text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table gateways (
  gateway_id varchar(64) primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  group_id varchar(64) references gateway_groups(group_id),
  name varchar(255) not null,
  status varchar(32) not null default 'unknown',
  last_seen_at timestamptz,
  kura_version varchar(64),
  os_version varchar(128),
  ip_addrs jsonb not null default '[]'::jsonb,
  mac_addrs jsonb not null default '[]'::jsonb,
  config_revision bigint,
  snapshot_id bigint,
  alert_count int not null default 0,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table gateway_tags (
  gateway_id varchar(64) not null references gateways(gateway_id) on delete cascade,
  tag varchar(128) not null,
  primary key (gateway_id, tag)
);

create table gateway_heartbeats (
  heartbeat_id bigserial primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  gateway_id varchar(64) not null references gateways(gateway_id) on delete cascade,
  ts timestamptz not null,
  online boolean not null,
  kura_version varchar(64),
  os_version varchar(128),
  ip_addrs jsonb not null default '[]'::jsonb,
  uptime_sec bigint,
  cpu_pct numeric(5,2),
  mem_pct numeric(5,2),
  disk_pct numeric(5,2),
  services jsonb not null default '{}'::jsonb,
  config_revision bigint,
  snapshot_id bigint,
  raw_payload jsonb not null default '{}'::jsonb
);

create table commands (
  command_id varchar(64) primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  gateway_id varchar(64) not null references gateways(gateway_id) on delete cascade,
  type varchar(64) not null,
  status varchar(32) not null,
  request_payload jsonb not null default '{}'::jsonb,
  reply_payload jsonb,
  initiator varchar(255),
  correlation_id varchar(128),
  requested_at timestamptz not null default now(),
  sent_at timestamptz,
  started_at timestamptz,
  finished_at timestamptz,
  timeout_at timestamptz,
  error_code varchar(64),
  error_message text
);

create table command_events (
  event_id bigserial primary key,
  command_id varchar(64) not null references commands(command_id) on delete cascade,
  event_type varchar(64) not null,
  status varchar(32),
  message text,
  created_at timestamptz not null default now()
);

create table config_templates (
  template_id varchar(64) primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  name varchar(255) not null,
  scope varchar(64) not null,
  target_model varchar(255),
  target_tags jsonb not null default '[]'::jsonb,
  revision bigint not null default 1,
  payload jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table config_template_versions (
  version_id bigserial primary key,
  template_id varchar(64) not null references config_templates(template_id) on delete cascade,
  revision bigint not null,
  payload jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  created_by varchar(255)
);

create table config_snapshots (
  snapshot_pk bigserial primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  gateway_id varchar(64) not null references gateways(gateway_id) on delete cascade,
  gateway_snapshot_id bigint not null,
  revision bigint not null default 0,
  payload jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  created_by varchar(255),
  source_command_id varchar(64),
  unique (gateway_id, gateway_snapshot_id)
);

create table audit_events (
  event_id bigserial primary key,
  tenant_id varchar(64) not null references tenants(tenant_id),
  actor varchar(255) not null,
  action varchar(128) not null,
  target_type varchar(64) not null,
  target_id varchar(64) not null,
  before_payload jsonb,
  after_payload jsonb,
  result varchar(32) not null,
  trace_id varchar(128),
  created_at timestamptz not null default now()
);

create index idx_gateways_tenant_status on gateways(tenant_id, status);
create index idx_gateways_tenant_group on gateways(tenant_id, group_id);
create index idx_heartbeat_gateway_ts on gateway_heartbeats(gateway_id, ts desc);
create index idx_commands_gateway_requested on commands(gateway_id, requested_at desc);
create index idx_commands_status_requested on commands(status, requested_at desc);
create index idx_audit_tenant_created on audit_events(tenant_id, created_at desc);
