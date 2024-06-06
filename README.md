# HOW TO RUN???

```
docker compose up --build -d
```

done(if docker was instaled before)!!!

then navigate to port 3000(or what you setted in .env) and use my service

development tools:

languages:

```
rust, typescript, javascript, (html, css) not languages lol
```

libs:

```
actix-web, async-graphql, vue, vite, seaorm
```

on top of that:

```
docker
```

tests:

| Test Case Name                                   | Test Case Id | Test Step | Test Data                                                                         | Expected Result   | Actual Result     | Remarks |
| ------------------------------------------------ | ------------ | --------- | --------------------------------------------------------------------------------- | ----------------- | ----------------- | ------- |
| Visit website(without login) and favorite advert | FAV_1        | 1         |                                                                                   | Relocate to login | Relocate to login | Pass    |
| Visit website(without login) and create advert   | CRE_1        | 2         |                                                                                   | Relocate to login | Relocate to login | Pass    |
| Visit website and register with wrong data       | REG_1        | 3         | Name: a<br>Email: a<br>Password: a <br> surname: a<br> phone: a <br> person: true | Show Error        | Show Error        | Pass    |
| Visit website and login with wrong info          | LOG_1        | 4         | email: a <br> password: a <br>                                                    | Show error        | Show error        | Pass    |
| Visit website and favorite advert after login    | FAV_2        | 5         |                                                                                   | Advert favorited  | Advert favorited  | Pass    |
