CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "users" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now()),
    "hashed_password" varchar NOT NULL,
    "first_name" varchar NOT NULL,
    "last_name" varchar NOT NULL,
    "email" varchar NOT NULL UNIQUE
);
CREATE TABLE "sessions" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now()),
    "user_id" uuid NOT NULL,
    "access_token" varchar NOT NULL,
    "refresh_token" varchar NOT NULL,
    "expires_at" timestamptz NOT NULL
);
CREATE TABLE "banks" (
    "id" varchar UNIQUE PRIMARY KEY NOT NULL,
    "user_id" uuid NOT NULL,
    "apk_key" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now())
);
CREATE TABLE "businesses" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "user_id" uuid NOT NULL,
    "name" varchar NOT NULL,
    "location" varchar NOT NULL,
    "geolocation" point NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now()),
    "lat" double precision NOT NULL,
    "lon" double precision NOT NULL
);
CREATE TABLE "Accounts" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "bank_id" varchar NOT NULL,
    "account_name" varchar NOT NULL,
    "account_number" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now())
);
CREATE TABLE "devices_accessible" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "device_id" varchar NOT NULL,
    "name" varchar NOT NULL,
    "device_type" varchar NOT NULL,
    "user_id" uuid NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now())
);
CREATE TABLE "devices" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "device_id" varchar NOT NULL,
    "name" varchar NOT NULL,
    "account_id" uuid NOT NULL,
    "device_type" varchar NOT NULL,
    "apk_key" varchar NOT NULL,
    "business_id" uuid NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now()),
    "main_id" uuid NOT NULL,
    "id_key" varchar NOT NULL,
    "price_key" varchar NOT NULL
);
CREATE TABLE "customers" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "first_name" varchar NOT NULL,
    "last_name" varchar NOT NULL,
    "public_key" varchar NOT NULL,
    "private_key" varchar NOT NULL,
    "bank_id" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now()),
    "public_id" uuid NOT NULL,
    "file_name" varchar NOT NULL
);
CREATE TABLE "payments" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "device_id" uuid NOT NULL,
    "amount" bigint NOT NULL,
    "customer_id" uuid NOT NULL,
    "user_id" uuid NOT NULL,
    "bank_id" varchar NOT NULL,
    "account_name" varchar NOT NULL,
    "account_number" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "updated_at" timestamptz NOT NULL DEFAULT (now())
);
-- Foreign keys
ALTER TABLE "sessions"
ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "banks"
ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "businesses"
ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "devices_accessible"
ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "devices"
ADD FOREIGN KEY ("account_id") REFERENCES "Accounts" ("id");
ALTER TABLE "devices"
ADD FOREIGN KEY ("business_id") REFERENCES "businesses" ("id");
ALTER TABLE "customers"
ADD FOREIGN KEY ("bank_id") REFERENCES "banks" ("id");
ALTER TABLE "payments"
ADD FOREIGN KEY ("device_id") REFERENCES "devices_accessible" ("id");
ALTER TABLE "payments"
ADD FOREIGN KEY ("customer_id") REFERENCES "customers" ("id");
ALTER TABLE "payments"
ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");