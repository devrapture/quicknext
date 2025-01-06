use std::collections::HashMap;

pub fn get_dependency_version_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // NextAuth.js
    map.insert("next-auth", "5.0.0-beta.25");
    map.insert("@auth/prisma-adapter", "^2.7.2");
    map.insert("@auth/drizzle-adapter", "^1.7.2");

    // Prisma
    map.insert("prisma", "^5.14.0");
    map.insert("@prisma/client", "^5.14.0");
    map.insert("@prisma/adapter-planetscale", "^5.14.0");

    // Drizzle
    map.insert("drizzle-kit", "^0.24.0");
    map.insert("drizzle-orm", "^0.33.0");
    map.insert("eslint-plugin-drizzle", "^0.2.3");
    map.insert("mysql2", "^3.11.0");
    map.insert("@planetscale/database", "^1.19.0");
    map.insert("postgres", "^3.4.4");
    map.insert("@libsql/client", "^0.9.0");

    // TailwindCSS
    map.insert("tailwindcss", "^3.4.3");
    map.insert("postcss", "^8.4.39");
    map.insert("prettier", "^3.3.2");
    map.insert("prettier-plugin-tailwindcss", "^0.6.5");

    // tRPC
    map.insert("@trpc/client", "^11.0.0-rc.446");
    map.insert("@trpc/server", "^11.0.0-rc.446");
    map.insert("@trpc/react-query", "^11.0.0-rc.446");
    map.insert("@trpc/next", "^11.0.0-rc.446");
    map.insert("@tanstack/react-query", "^5.50.0");
    map.insert("superjson", "^2.2.1");
    map.insert("server-only", "^0.0.1");

    map
}