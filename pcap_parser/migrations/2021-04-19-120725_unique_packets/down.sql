-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS deduplication_index;

ALTER TABLE packets_pcap_parser
DROP COLUMN IF EXISTS payload_hash