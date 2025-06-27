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
  - [ ] * add trait for codes
  - [ ] * implement basic form which is an enum of primitive types
  - [ ] * implement `None` code with `done` form

Improvements
------------

- [ ] Setup login cookie expiration and implement remember-me to increate the time to 10 days (see gepeto).
- [ ] Force authentication to every call except login by hiding db in authentication (except for a simplified db for login)
- [ ] Unify treatment of errors and redirects (redirect on error is not working)
