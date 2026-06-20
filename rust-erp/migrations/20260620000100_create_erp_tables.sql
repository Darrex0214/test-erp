CREATE TABLE IF NOT EXISTS journal_entries (
    id UUID PRIMARY KEY,
    description TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    posted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS journal_postings (
    id BIGSERIAL PRIMARY KEY,
    journal_entry_id UUID NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id UUID NOT NULL,
    side TEXT NOT NULL CHECK (side IN ('debit', 'credit')),
    amount NUMERIC NOT NULL,
    currency TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY,
    sku TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    stock BIGINT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY,
    number TEXT NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('draft', 'issued', 'paid', 'canceled')),
    issued_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS invoice_lines (
    id BIGSERIAL PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price NUMERIC NOT NULL,
    currency TEXT NOT NULL
);
