* means that the task is essential for an MVP

Features
--------

- DB/API
  - [ ] delete note
- Frontend
  - [ ] edit note
  - [ ] add attributes to notes view
  - [ ] delete note
  - [ ] hide or change the apperence of done notes
  - [ ] breadcrumbs for ancestors
- Coding
  - [ ] * implement done with lua

Improvements
------------

- [ ] Setup login cookie expiration and implement remember-me to increate the time to 10 days (see gepeto).
- [ ] Force authentication to every call except login by hiding db in authentication (except for a simplified db for login)
- [ ] Unify treatment of errors and redirects (redirect on error is not working)
- [ ] Unify return errors to DbError (it often uses sqlx:Error)
