alter table gateways add column enabled boolean not null default true;

create table gateway_credentials (
  credential_id bigserial primary key,
  gateway_id varchar(64) not null references gateways(gateway_id) on delete cascade,
  tenant_id varchar(64) not null references tenants(tenant_id),
  secret_hash varchar(255) not null,
  enabled boolean not null default true,
  created_at timestamptz not null default now(),
  rotated_at timestamptz,
  last_used_at timestamptz
);

create index idx_gateway_credentials_gateway on gateway_credentials(gateway_id, credential_id desc);
