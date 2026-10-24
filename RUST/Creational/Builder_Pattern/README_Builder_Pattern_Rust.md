# Builder Design Pattern in Rust

> **Topic:** Creational Design Pattern\
> **Language:** Rust\
> **Example:** Constructing a `People` value with a fluent API\
> **Purpose:** A practical knowledge base covering intent, structure,
> diagrams, trade-offs, implementation, testing, and system-design
> applications.

------------------------------------------------------------------------

## 1. What is the Builder Pattern?

The **Builder Pattern** is a creational design pattern that separates
the process of constructing an object from the final object itself. It
is useful when an object has many fields, optional values, validation
rules, or multiple construction steps.

Instead of passing many arguments to one large constructor, the caller
builds the value step by step:

``` rust
let person = People::new()
    .name("Manish Sah".to_string())
    .id(932)
    .email("manish@example.com".to_string())
    .description("Rust developer".to_string());
```

This style is often called a **fluent interface** because each method
returns a value on which the next method can be called.

### The core idea

-   **Without a builder:** the caller must understand a constructor's
    full argument list and ordering.
-   **With a builder:** each choice is named, chained, and easier to
    read.
-   **With validation:** the builder can check whether required values
    are present and valid before producing the final object.

> **Important distinction:** Your current implementation is a valid
> *builder-style fluent API*, but `People` is acting as both the builder
> and the final data object. A classic Builder Pattern commonly uses a
> separate `PeopleBuilder` type and a final `People` type. Both
> approaches can be useful; choose based on complexity.

------------------------------------------------------------------------

## 2. The problem the pattern solves

Imagine a user/profile object has many fields:

``` rust
struct People {
    name: String,
    id: i32,
    email: String,
    description: String,
}
```

A positional constructor might look like this:

``` rust
// Illustrative example; not implemented in this project.
People::new(
    "Manish Sah".to_string(),
    932,
    "manish@example.com".to_string(),
    "Rust developer".to_string(),
);
```

This can become difficult to read as fields grow. It is also easy to mix
up arguments of the same type.

A builder expresses intent through method names:

``` rust
People::new()
    .name("Manish Sah".to_string())
    .id(932)
    .email("manish@example.com".to_string())
    .description("Rust developer".to_string());
```

### When to consider it

-   An object has many configuration options.
-   Several fields are optional.
-   Construction requires validation or normalization.
-   Different callers need different combinations of options.
-   A configuration object would otherwise need many constructors.
-   You want a readable API for HTTP clients, database settings, UI
    components, or application services.

### When it may be unnecessary

For a small type with one or two required fields, a normal constructor
or struct literal may be simpler. A builder adds another abstraction and
should solve a real construction problem.

------------------------------------------------------------------------

## 3. Pattern classification and intent

  -----------------------------------------------------------------------
  Property                            Description
  ----------------------------------- -----------------------------------
  Pattern category                    Creational

  Main intent                         Construct a complex value
                                      incrementally

  Main participants                   Builder, Concrete Builder, Product;
                                      optionally Director

  Typical result                      A fully configured object

  Common API style                    Chainable methods returning `self`
                                      or `&mut self`

  Main benefit                        Readable, flexible construction

  Main cost                           Extra types and methods; possible
                                      runtime validation
  -----------------------------------------------------------------------

### Related patterns

-   **Factory Method:** chooses or delegates which concrete product to
    create.
-   **Abstract Factory:** creates families of related products.
-   **Prototype:** creates a new object by cloning an existing one.
-   **Dependency Injection:** supplies dependencies from outside a
    component; it can be used alongside a builder.
-   **Fluent Interface:** an API design style that enables method
    chaining. It is not itself the same thing as the Builder Pattern.

------------------------------------------------------------------------

## 4. UML-style class diagram

The classic Builder Pattern separates the builder from the product:

``` mermaid
classDiagram
    class People {
        +String name
        +i32 id
        +String email
        +String description
    }

    class PeopleBuilder {
        -Option~String~ name
        -Option~i32~ id
        -Option~String~ email
        -Option~String~ description
        +name(String) PeopleBuilder
        +id(i32) PeopleBuilder
        +email(String) PeopleBuilder
        +description(String) PeopleBuilder
        +build() Result~People, BuildError~
    }

    PeopleBuilder ..> People : creates
```

**How to read it:** the caller configures `PeopleBuilder`. The builder
validates its stored values and creates a `People` product when
`build()` is called.

Your current implementation merges these two roles into one type. That
is simpler for a small example, but separating them is often easier to
maintain as validation and construction logic grow.

------------------------------------------------------------------------

## 5. Construction flow diagram

``` mermaid
flowchart TD
    A([Start]) --> B[PeopleBuilder::new]
    B --> C[Set name]
    C --> D[Set id]
    D --> E[Set email]
    E --> F[Set description]
    F --> G{Required values valid?}
    G -- No --> H[Return a build error]
    G -- Yes --> I[Create People]
    I --> J([Use the completed value])
    H --> K([Fix configuration and retry])
```

The builder is a **construction helper**, not necessarily a runtime
service. Once the final `People` value is created, normal application
code should usually work with that value rather than continue to depend
on the builder.

------------------------------------------------------------------------

## 6. Sequence diagram

``` mermaid
sequenceDiagram
    actor Caller
    participant Builder as PeopleBuilder
    participant Product as People

    Caller->>Builder: new()
    Builder-->>Caller: empty builder
    Caller->>Builder: name("Manish Sah")
    Builder-->>Caller: updated builder
    Caller->>Builder: id(932)
    Builder-->>Caller: updated builder
    Caller->>Builder: email(...)
    Builder-->>Caller: updated builder
    Caller->>Builder: description(...)
    Builder-->>Caller: updated builder
    Caller->>Builder: build()
    Builder->>Builder: validate fields
    Builder->>Product: construct People
    Product-->>Caller: completed value
```

------------------------------------------------------------------------

## 7. Data-flow diagram (DFD)

This is a conceptual **Level 0 data-flow diagram**, not a network
deployment diagram.

``` mermaid
flowchart LR
    U[Caller / Application] -->|Field values and options| B((People Builder))
    B -->|Validate required fields| V{Validation}
    V -->|Invalid configuration| E[Build error]
    V -->|Valid configuration| P[People value]
    P -->|Completed object| U
```

The caller supplies configuration data; the builder validates and
transforms it into the final product. The builder should not silently
hide validation failures when correctness matters.

------------------------------------------------------------------------

## 8. Rust implementation: separate builder and product

The following version demonstrates a more explicit Builder Pattern. It
uses `Option<T>` in the builder so that missing required fields can be
detected, and `Result` to report errors.

``` rust
#[derive(Debug, PartialEq, Eq)]
pub struct People {
    pub name: String,
    pub id: i32,
    pub email: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    MissingName,
    MissingId,
    MissingEmail,
}

#[derive(Debug, Default)]
pub struct PeopleBuilder {
    name: Option<String>,
    id: Option<i32>,
    email: Option<String>,
    description: Option<String>,
}

impl PeopleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Result<People, BuildError> {
        let name = self.name.ok_or(BuildError::MissingName)?;
        let id = self.id.ok_or(BuildError::MissingId)?;
        let email = self.email.ok_or(BuildError::MissingEmail)?;
        let description = self.description.unwrap_or_default();

        Ok(People {
            name,
            id,
            email,
            description,
        })
    }
}

fn main() -> Result<(), BuildError> {
    let person = PeopleBuilder::new()
        .name("Manish Sah")
        .id(932)
        .email("manish@example.com")
        .description("Rust developer")
        .build()?;

    println!("{person:#?}");
    Ok(())
}
```

### Why these choices?

-   `Option<T>` distinguishes "not configured" from a supplied value.
-   `build(self)` consumes the builder, so the builder cannot
    accidentally be reused after building.
-   `Result<People, BuildError>` makes failure explicit.
-   `impl Into<String>` accepts both `String` and string slices such as
    `&str`.
-   `description.unwrap_or_default()` makes the description optional and
    defaults it to an empty string.
-   `#[derive(Debug)]` enables debug formatting with `{:?}` or `{:#?}`.

**Design note:** In a real application, validate email format and
domain-specific ID constraints according to the product's requirements.
A non-empty string alone does not prove that an email address is valid
or verified.

------------------------------------------------------------------------

## 9. Understanding the original implementation

Your version uses `People` for both configuration and the final data:

``` rust
impl People {
    fn new() -> Self {
        Self {
            name: String::new(),
            id: 0,
            email: String::new(),
            description: String::new(),
        }
    }

    fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
```

### What `mut self` and `-> Self` mean

-   `self` is the current owned value.
-   `mut self` allows the method to modify that value.
-   `self.name = name` replaces the field.
-   `self` returns the updated value, allowing another method call.

This is **ownership-based method chaining**. It does not require a
mutable variable at the call site because each method takes ownership
and returns the next value.

### What to improve

1.  **Avoid placeholder values for required fields.** An empty name, ID
    `0`, or empty email may look like a valid `People` even when
    construction is incomplete.
2.  **Use a separate builder for more complex types.** It distinguishes
    incomplete configuration from a valid product.
3.  **Use `&str`-friendly APIs.** `impl Into<String>` reduces repeated
    `.to_string()` calls.
4.  **Use `build()` when validation is required.** This makes the point
    of final construction explicit.
5.  **Avoid logging personal data.** Debug-printing the whole struct can
    expose names or email addresses in logs.
6.  **Use a suitable ID type.** If IDs are positive identifiers,
    consider a domain-specific type or `u64` after deciding the actual
    requirements. Numeric IDs should not be formatted with leading zeros
    to imply a different numeric value.

------------------------------------------------------------------------

## 10. Ownership and API design choices

Rust builders commonly use one of these method styles:

  --------------------------------------------------------------------------------------------------
  Style                   Example signature                                  How it behaves
  ----------------------- -------------------------------------------------- -----------------------
  Consuming builder       `fn name(mut self, value: String) -> Self`         Takes ownership and
                                                                             returns the updated
                                                                             builder; ergonomic for
                                                                             chaining

  Mutable-reference       `fn name(&mut self, value: String) -> &mut Self`   Mutates in place;
  builder                                                                    useful when configuring
                                                                             a builder across
                                                                             multiple statements

  Borrowing build         `fn build(&self) -> Result<People, BuildError>`    Builds without
                                                                             consuming the builder;
                                                                             may allow repeated
                                                                             builds

  Consuming build         `fn build(self) -> Result<People, BuildError>`     Consumes the builder
                                                                             and makes the
                                                                             finalization step
                                                                             explicit
  --------------------------------------------------------------------------------------------------

Neither consuming nor mutable-reference builders are universally
superior. Choose based on whether re-use, cloning, and ownership
semantics are important to the API.

------------------------------------------------------------------------

## 11. System-design perspective

The Builder Pattern is mainly an **object-construction technique**, not
a complete system architecture. Its system-design value appears when
complex configuration needs a clear boundary between input and a valid
application object.

### Example: building a service configuration

``` mermaid
flowchart TD
    A[Environment variables / config file] --> B[Configuration parser]
    B --> C[ServiceConfigBuilder]
    C --> D{Validate configuration}
    D -->|Invalid| E[Startup fails with clear error]
    D -->|Valid| F[ServiceConfig]
    F --> G[HTTP server]
    F --> H[Database pool]
    F --> I[Logging / tracing]
```

A builder can centralize configuration validation before services start.
For example, a database configuration might need a host, port, database
name, and connection limit. It should reject invalid combinations before
opening connections.

### Responsibilities and boundaries

-   **Input layer:** reads environment variables, config files, or user
    input.
-   **Builder:** gathers values, applies defaults, and validates
    construction rules.
-   **Product:** holds the valid configuration or domain object.
-   **Application services:** consume the completed product.
-   **Infrastructure:** starts external resources, such as a database
    pool, using the validated configuration.

Do not put unrelated application workflows into the builder. It should
construct the object, not become a god object that also connects to
databases, sends emails, or handles HTTP requests.

------------------------------------------------------------------------

## 12. Benefits and trade-offs

  -----------------------------------------------------------------------
  Benefits                            Trade-offs
  ----------------------------------- -----------------------------------
  Readable named construction steps   More boilerplate and types

  Supports optional fields and        Poor defaults can hide missing data
  defaults                            

  Central place for validation        Validation rules can become overly
                                      complex

  Avoids huge positional constructors Can be unnecessary for simple
                                      structs

  Supports staged or constrained      Compile-time typestate builders add
  construction                        complexity

  Easier to extend with new optional  API design needs care to avoid
  settings                            breaking callers
  -----------------------------------------------------------------------

### Performance

A straightforward Rust builder usually has negligible runtime overhead
after optimization: methods move values and the builder is generally
optimized away. However, allocations inside fields such as `String`
still have their normal costs. Do not assume the builder itself
eliminates allocations.

------------------------------------------------------------------------

## 13. Testing strategy

Test both successful construction and invalid configurations.

``` rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_people_with_all_required_fields() {
        let person = PeopleBuilder::new()
            .name("Manish Sah")
            .id(932)
            .email("manish@example.com")
            .build()
            .expect("all required fields were supplied");

        assert_eq!(person.name, "Manish Sah");
        assert_eq!(person.id, 932);
        assert_eq!(person.email, "manish@example.com");
        assert_eq!(person.description, "");
    }

    #[test]
    fn rejects_missing_name() {
        let result = PeopleBuilder::new()
            .id(932)
            .email("manish@example.com")
            .build();

        assert_eq!(result, Err(BuildError::MissingName));
    }
}
```

Run tests from the project root:

``` bash
cargo test
```

If this is a standalone `.rs` file rather than a Cargo project, either
put the code in a Cargo package for the test workflow or compile the
example directly with `rustc` after removing the test-only setup as
appropriate.

**Further tests to add:** missing ID, missing email, empty values,
boundary IDs, and any real validation rules. Keep tests focused on
observable behavior, not internal implementation details.

------------------------------------------------------------------------

## 14. Common mistakes

-   Treating a fluent chain as proof that the classic Builder Pattern
    has been fully implemented.
-   Using default values such as `0` or `""` to represent "not provided"
    when those values could be legitimate.
-   Forgetting to validate required fields.
-   Returning a product that is only partially configured.
-   Making the builder responsible for I/O or unrelated business
    operations.
-   Exposing sensitive fields through `Debug` logs.
-   Adding a builder to every struct even when a simple constructor is
    clearer.
-   Using `unwrap()` in production construction paths where a useful
    error should be returned.

------------------------------------------------------------------------

## 15. Variations

### A. Optional fields with defaults

A builder can initialize optional fields with sensible defaults and let
callers override them. Defaults should be documented and safe.

### B. Required fields in the constructor

``` rust
// Alternative API idea:
PeopleBuilder::new("Manish Sah", 932)
    .email("manish@example.com")
    .build();
```

This makes some invalid states impossible to represent because required
values are supplied immediately.

### C. Typestate builder

A typestate builder uses generic marker types to encode construction
progress in the type system. It can make `build()` available only after
required fields have been set. This can be valuable for safety-critical
APIs, but it introduces more advanced generics and is often unnecessary
for ordinary application structs.

### D. Director

In the classic GoF description, an optional **Director** defines a
repeatable construction recipe using a builder. For example,
`create_admin_profile(builder)` could configure a standard profile. In
idiomatic Rust applications, this role is often just a helper function
and does not always need a separate `Director` type.

------------------------------------------------------------------------

## 16. Builder vs. Factory

  -----------------------------------------------------------------------
  Question                Builder                 Factory
  ----------------------- ----------------------- -----------------------
  Primary concern         How to assemble one     Which product or
                          product                 implementation to
                                                  create

  Typical API             Chain options, then     `create(...)` or a
                          `build()`               factory method

  Best fit                Many options or         Choosing among product
                          validation steps        types

  Can be combined?        Yes                     Yes
  -----------------------------------------------------------------------

For example, a factory could choose between `PostgresStorage` and
`MemoryStorage`, while a builder configures the selected storage's
connection pool settings.

------------------------------------------------------------------------

## 17. Checklist for reviewing a builder

-   [ ] Is the object complex enough to justify a builder?
-   [ ] Are required fields clearly distinguished from optional fields?
-   [ ] Are defaults safe and documented?
-   [ ] Does `build()` validate all important invariants?
-   [ ] Are errors meaningful and recoverable?
-   [ ] Is the final product valid after construction?
-   [ ] Does the builder have one responsibility?
-   [ ] Are sensitive values kept out of logs?
-   [ ] Are successful and failure cases tested?
-   [ ] Would a simple constructor be clearer?

------------------------------------------------------------------------

## 18. Further reading

-   [Rust Book ---
    Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
-   [Rust Book ---
    Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
-   [Rust Reference](https://doc.rust-lang.org/reference/)
-   [Refactoring.Guru ---
    Builder](https://refactoring.guru/design-patterns/builder)
-   [Design Patterns: Elements of Reusable Object-Oriented
    Software](https://en.wikipedia.org/wiki/Design_Patterns)

------------------------------------------------------------------------

## 19. Summary

The Builder Pattern helps construct complex objects through readable,
controlled steps. Your original `People::new().name(...).id(...)` chain
demonstrates the fluent construction idea. For a more robust design,
separate `PeopleBuilder` from `People`, represent unprovided values
explicitly, validate them in `build()`, and return a `Result` when
construction can fail.

**Rule of thumb:** use a builder when construction complexity is real;
keep it small, explicit, and focused on producing a valid object.
