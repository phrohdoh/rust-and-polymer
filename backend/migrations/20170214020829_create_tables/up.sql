create table transactions (
    id INTEGER NOT NULL PRIMARY KEY,
    payee_id INTEGER NOT NULL,
    amount REAL NOT NULL
);

create table payees (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);