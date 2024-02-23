use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// object macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

// struct Context {
//     // Use your real database pool here.
//     pool: DatabasePool,
// }

// // To make our context usable by Juniper, we have to implement a marker trait.
// impl juniper::Context for Context {}

struct Query;

#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    // context = Context,
)]

impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn human(id: String) -> FieldResult<Human> {
        // Get a db connection.
        // let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        // let human = connection.find_human(&id)?;
        let human: Human = Human {
            id: "23inn32".to_string(),
            name: "Hey Sam".to_string(),
            appears_in: vec![],
            home_planet: "Jupiter".to_owned(),
        };
        // Return the result.
        Ok(human)
    }
}

// Now, we do the same for our Mutation type.

struct Mutation;

#[graphql_object(
    // context = Context,
    // If we need to use `ScalarValue` parametrization explicitly somewhere
    // in the object definition (like here in `FieldResult`), we could
    // declare an explicit type parameter for that, and specify it.
    // scalar = S: ScalarValue + Display,
)]
impl Mutation {
    fn createHuman<S: ScalarValue + Display>(
        // context: &Context,
        new_human: NewHuman,
    ) -> FieldResult<Human> {
        // let db = context
        //     .pool
        //     .get_connection()
        //     .map_err(|e| e.map_scalar_value())?;

        let human: Human = Human {
            id: "23inn32".to_string(),
            name: "Hey Sam".to_string(),
            appears_in: vec![],
            home_planet: "Jupiter".to_owned(),
        };

        Ok(human)
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
// type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription>;
