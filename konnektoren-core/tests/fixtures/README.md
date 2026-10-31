# Legacy session fixtures

`legacy-session.json` and `legacy-session.yml` were captured with the required
`ChallengeResult` field, before changing it to `Option<ChallengeResult>`.
They contain a session with an answered challenge, timestamps, history, and XP.
The MessagePack fixture is the same legacy JSON snapshot encoded as named maps.
Do not regenerate these fixtures from the current optional-result structs.

The compatibility tests check loading old saves and continuing play, all legacy
result variants, and preservation of the existing JSON shape for present results.
A newly absent result serializes as null; older required-result readers cannot
load that new state. Backward loading is supported, rollback to an old client is
not guaranteed. Browser local storage and Google Drive backups use JSON.

Compact positional MessagePack has a pre-existing limitation with configuration
fields using `skip_serializing_if`: omitted fields shift array positions. Use
named maps for saved state. These tests do not claim positional compatibility.
