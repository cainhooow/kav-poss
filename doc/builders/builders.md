## Usando um role builder ou builders

```rs
// criar um builder
let builder = RoleBuilder::new("ROLE_NAME");
// construir struct role
builder.build();
// NewRole { name: "ROLE_NAME", description: None }
```

## Criando com campos opcionais

```rs
let builder = RoleBuilder::new("ROLE_NAME")
.description(Some("My role description".to_string()))

builder.build();
// NewRole { name: "ROLE_NAME", description: Some("My role description") }

```

OBS: O uso de `.description(Option<String>)` poderia ser `.description(<String>)`, mas
para facilitar verificadores extras no uso do builder, o parametro é um argumento
opcional, podendo mudar futuramente para obrigatorio `.description(<String>)`

## Exemplo de validação extra com erro

```rs
// recebe de algum outro lugar = request, etc...
let role_description = Some(String::from("Hello world"));
let role_builder = RoleBuilder::new("HELLO_ROLE");

if let Some(description) = role_description {
    role_builder.description(Some(description)); // role_builder movido aqui
}

let role = role_builder.build() // ERRO - role_builder foi movido para dentro da validação;
```

### Para solucionar esse problema, você precisa "herdar a variavel" e retornar builder dentro da validação, isso explica o motivo de `.description(<String>)` não ser usado, pois teria que criar validações extras

```rs
// recebe de algum outro lugar = request, etc...
let role_description = Some(String::from("Hello world"));
let role_builder = RoleBuilder::new("HELLO_ROLE");

let role_builder = if let Some(description) = role_description {
    role_builder.description(Some(description)) // retorna builder
} else {
    role_builder // retorna apenas builder sem description
}

let role = role_builder.build() // NewRole {...}
```
