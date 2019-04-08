# Prisma Rust Quiz: Type Golf

This weeks rust quiz is more of a test could we utilize Rust's type system
better for our needs.

In the `main.rs` we'll see a set of types for database querying:

- `Model` to store a row information
- `Field` to store a column info for the `Model`

Additionally we have `RecordFinder`, a type that has one purpose: finding
exactly one record from the database. Our current solution shows how to load the
model from the given `model.json` and how to construct a `RecordFinder` for
querying.

What is wrong with this approach is that we cannot be sure is the `Field`
returned from the `Model` unique without checking its status on runtime. What
would be better is that the field could store it's uniqueness in the type level,
and we would make it as hard as possible for the user to create `RecordFinder`
with a field that is not unique.

Please do some research could we map the JSON data directly to the proper types,
making it hard for anybody to misuse the `RecordFinder`.
