INSERT INTO settings
    (id, value, enabled)
VALUES
    ('runtime.daemon.enable', null, true),
    ('workspace.load_last', null, false),
    ('rpc.enabled', null, true),
    ('rpc.client_id', '', true),
    ('rpc.enable_idle', null, true),
    ('rpc.show_current_workspace', null, true),
    ('rpc.show_current_file', null, true),
    ('language.locale', null, true),
    ('appearance.font.primary', 'Roboto', true),
    ('appearance.font.monospace', 'Roboto Mono', true),
    ('appearance.font.dyslexic.primary', 'OpenDyslexic', true),
    ('appearance.font.dyslexic.monospace', 'OpenDyslexic Mono', true),
    ('appearance.theme')
    ON CONFLICT IGNORE;