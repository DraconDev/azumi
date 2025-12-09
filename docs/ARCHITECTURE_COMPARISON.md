# Architecture Comparison: Azumi Starter vs. Lesson 19

The user asked if `azumi-starter` is "doing it wrong". The anwer is **No**. It is using a **Microservice Architecture**, while Lesson 19 teaches a **Monolithic Architecture**.

## 1. The Core Difference: "Who Checks the ID?"

| Feature           | Lesson 19 (Monolith)                          | Azumi Starter (Microservices)                                    |
| :---------------- | :-------------------------------------------- | :--------------------------------------------------------------- |
| **Auth Check**    | **Local**: The app checks the cookie itself.  | **Remote**: The app asks a separate **Auth Service** (via gRPC). |
| **Session Store** | **Cookie / Memory**: Simple and fast.         | **Redis**: Distributed and scalable.                             |
| **Complexity**    | **High Simplicity**: One app does everything. | **High Scalability**: Auth can run on a separate server.         |
| **Best For**      | Learning, MVPs, Small-Med Apps.               | Large Enterprise, Multi-Team projects.                           |

## 2. Code Comparison

### Lesson 19 (What we just built)

We "Trust the Cookie" (or check a local database):

```rust
// Local check
if let Some(cookie) = jar.get("azumi_user") {
    // We are the authority!
    req.extensions_mut().insert(User { ... });
}
```

### Azumi Starter (What is in the repo)

It "Asks the Authority" via gRPC:

```rust
// Remote check
let response = client.validate_session(Request::new(ValidateSessionRequest {
    session_token: session_token
})).await?;

if response.valid {
    // The Auth Service says they are good
    req.extensions_mut().insert(AuthenticatedUser { ... });
}
```

## 3. Dependency Status

`azumi-starter` is indeed slightly outdated regarding **Azumi Phase 3 Features**:

-   **Signed State (`#[azumi::live]`)**: It might not be using the latest signed state protection.
-   **Asset Pipeline**: It uses standard static files, not the hashed `assets!` macro.
-   **Dependency**: It points to `git` instead of your local path.

## 4. Recommendation

-   **For Learning**: Stick with Lesson 19. It's easier to understand.
-   **For Production**: If you are building a huge app, `azumi-starter` is a better _template_ (because of gRPC/SQLx setup), but you should **update it** to use the new Azumi features (Signed State, Assets) you learned here.
