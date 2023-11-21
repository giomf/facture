#import "@preview/tablex:0.0.6": tablex, colspanx, vlinex, hlinex
#let data = json("data.json")

#let invoice(
  id: str,
  date: str,
  total_price: int,
  customer: (
    id: str,
    contact: (name: str, surname: str, phone: str, mail: str),
    address: (street: str, house_number: str, city: str, postcode: str),
  ),
  business: (
    contact: (name: str, surname: str, phone: str, mail: str),
    address: (street: str, house_number: str, city: str, postcode: str),
    payment: (bank: str, iban: str, bic: str),
    legal: (court: str, tax_number: str),
  ),
  item_keys: (),
  items: ((position: int, name: str, amount: int, price: int, total_price: int)),
) = {
  set text(size: 11pt)
  set page(paper: "a4", header-ascent: -100%, header: align(right)[
    #business.contact.name #business.contact.surname \
    #business.address.street #business.address.house_number\
    #business.address.postcode #business.address.city \
    #business.contact.phone \
    #link("mailto:" + business.contact.mail) \
  ], footer-descent: 10%, footer: [
    #line(length: 100%)
    #grid(columns: (10%, 40%, 20%, 30%), [
      Bank:\
      IBAN:\
      BIC:
    ], [
      #business.payment.bank\
      #business.payment.iban\
      #business.payment.bic
    ], align(left)[
      Amtsgericht:\
      Steuernummer:
    ], align(right)[
      #business.legal.court\
      #business.legal.tax_number
    ])
  ])

  place(dy: 25mm)[
    #align(left)[
      #customer.contact.name #customer.contact.surname \
      #customer.address.street #customer.address.house_number \
      #customer.address.postcode #customer.address.city \
    ]
  ]

  v(25%)
  grid(columns: (1fr, 1fr, 1fr), [], [], [
    *Rechnung*
    #grid(columns: (1fr, 1fr), [
      Rechnungsnummer:\
      Kundennummer:\
      Rechnungsdatum:\
    ], align(right)[
      #id \
      #customer.id\
      #date \
    ])
  ])

  line(length: 100%)

  tablex(
    auto-lines: false,
    columns: (0.5fr, 3.25fr, 0.75fr, 1fr, 1.5fr),
    align: (center, left, right, right, right),
    fill: (_, row) => {
      if calc.rem(row, 2) == 1 {
        gray.lighten(75%)
      }
    },
    ..item_keys.map(
      header => align(left)[*#header*]
    ),
    ..items.map(row => row.values()).flatten(),
    colspanx(items.len() + 1,)[],
    underline()[*#total_price*],
  )

  "Gemäß § 19 UStG wird keine Umsatzsteuer berechnet."
}

#show: doc => invoice(
  id: data.id,
  date: data.date,
  total_price: data.total_price,
  customer: data.customer,
  business: data.business,
  item_keys: data.item_keys,
  items: data.items,
)