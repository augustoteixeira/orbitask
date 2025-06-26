Features
--------

- DB/API
  - [ ] create new note
  - [ ] delete note
  - [ ] get all root notes
- Frontend
  - [ ] create new note
  - [ ] delete note
  - [ ] hide done notes
  - [ ] breadcrumbs for ancestors
  - [ ] root notes at `/`
- Coding
  - [ ] add trait for codes
  - [ ] implement basic form which is an enum of primitive types
  - [ ] implement `None` code with `done` form

Improvements
------------

- [ ] Setup login cookie expiration and implement remember-me to increate the time to 10 days (see gepeto).
- [ ] Force authentication to every call except login by hiding db in authentication (except for a simplified db for login)
- [ ] Unify treatment of errors and redirects (redirect on error is not working)
