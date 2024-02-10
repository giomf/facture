CREATE TABLE items(
    id INTEGER PRIMARY KEY NOT NULL,
    position INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    amount INTEGER NOT NULL,
    price REAL NOT NULL,
    invoice_id INTEGER REFERENCES invoices(id) NOT NULL,
    foreign key (invoice_id) references invoices(id)
)
