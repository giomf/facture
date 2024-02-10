CREATE TABLE invoices(
    id INTEGER PRIMARY KEY NOT NULL,
    customer_id INTEGER REFERENCES customers(id) NOT NULL,
    foreign key (customer_id) references customers(id)
)
