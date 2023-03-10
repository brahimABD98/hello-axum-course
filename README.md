# hello axum course

this is my first axum project following the great tutorial from [Brooks builds 's axum playlist on youtube](https://www.youtube.com/watch?v=tzFdrLxv1Bg&list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS)

## how to run

### postgres setup with docker

```cmd
docker-compose up -d
```

### to run the container on the background(-d) and wait for the container to be ready(--wait)

```cmd
docker-compose up -d --wait
```

### add .env file with the following content (project map below)

```env
DATABASE_URL=postgres://postgres:keyoarbcat@localhost:5432/postgres
```

### to run the project

```cmd
cargo run
```

### run with live reload

```cmd
cargo watch -x run
```

### project map

```cmd
    ├── src
    │   ├── main.rs
    │   ├── routes
    │   ├── lib.rs
    │   ├── data
    │   │   ├── .env
    ├── database
    │   ├── init.sql
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── docker-compose.yml

```
