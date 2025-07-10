Work sprint
-----------

A) Editorial work
  - Comments
    - currently broken
  - Functionality
    - Create subnote effect
  - Lua
    - codes: journal, article, referee
B) Monthly payments
  - Create a pall endpoint

Features
--------

- DB/API
  - delete note
  - build alarms
    - add alarm to sql
    - add db_function to call expired alarms (and add to all endpoints, including "/")
    - build watchdog
- Frontend
  - * edit note
  - delete note
  - hide or change the apperence of done notes
  - breadcrumbs for ancestors
- Coding

Improvements
------------

- make all endpoints return a single enum that implements Responder.
- make tests call the handler directly (instead of calling an endpoint) through Client (these tests look more like unit tests afterall).
- improve flash with a nested structure (perhaps html?) instead of a string.
- Setup login cookie expiration and implement remember-me to increate the time to 10 days (see gepeto).
- Force authentication to every call except login by hiding db in authentication (except for a simplified db for login)
- Unify treatment of errors and redirects (redirect on error is not working)
- Unify return errors to DbError (it often uses sqlx:Error)
- make ui more compact, with all information
