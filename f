// RustORM Schema
// Documentation: https://github.com/Fakhir-Israr-200219/RustORM

model User {
    id    Int    @id @default(autoincrement())
    name  String
    place String?
    hujra String?
    lips String?
    email String @unique
    posts Post[]
}

model Post {
    id      Int    @id @default(autoincrement())
    title   String
    content String?
    user_id Int
    user    User @relation(fields: [user_id], references: [id])
}