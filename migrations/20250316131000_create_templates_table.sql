CREATE TABLE IF NOT EXISTS templates (
    unid        UUID                    PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    title       CHARACTER VARYING(255)  NOT NULL,
    description TEXT                    NOT NULL,
    created_at  TIMESTAMPTZ             NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ             NOT NULL DEFAULT now()
);


-- Insert data into templates table
INSERT INTO templates (unid, title, description) VALUES
('00000000-0000-0000-0000-000000000001', 'Value #1', 'Description for value #1'),
('00000000-0000-0000-0000-000000000002', 'Value #2', 'Description for value #2'),
('00000000-0000-0000-0000-000000000003', 'Value #3', 'Description for value #3'),
('00000000-0000-0000-0000-000000000004', 'Value #4', 'Description for value #4');


