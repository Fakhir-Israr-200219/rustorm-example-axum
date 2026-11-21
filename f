Bhai, yeh lo **complete Postman collection** jo tumhare saare endpoints test karegi. Seed data + eager loading verification dono cover hain.

## Step 1: Pehle Seed Data Banao

### POST /users
```json
{
    "name": "Ali Khan",
    "email": "ali@example.com",
    "place": "Lahore",
    "age": 28,
    "bio": "Full-stack developer"
}
```

### POST /users (2nd user)
```json
{
    "name": "Sara Ahmed",
    "email": "sara@example.com",
    "place": "Karachi",
    "age": 25,
    "bio": "Backend engineer"
}
```

### POST /posts (user_id = jo pehle user ka ID aaya)
```json
{
    "title": "Rust ORM Guide",
    "content": "Complete guide to building ORMs in Rust",
    "published": true,
    "view_count": 150,
    "user_id": 1
}
```

### POST /posts (2nd post)
```json
{
    "title": "Axum Integration",
    "content": "How to integrate RustORM with Axum",
    "published": true,
    "view_count": 89,
    "user_id": 1
}
```

### POST /posts (Sara's post)
```json
{
    "title": "Database Design Patterns",
    "content": "Best practices for relational databases",
    "published": false,
    "view_count": 42,
    "user_id": 2
}
```

⚠️ **Note:** `created_at` field abhi required hai create structs mein. Agar error aaye toh add karo: `"created_at": "2025-01-15T10:30:00"`

## Step 2: Eager Loading Test — Expected Responses

### GET /users/with-posts
**Expected Response:**
```json
[
    {
        "id": 1,
        "name": "Ali Khan",
        "email": "ali@example.com",
        "place": "Lahore",
        "age": 28,
        "bio": "Full-stack developer",
        "posts": [
            {
                "id": 1,
                "title": "Rust ORM Guide",
                "content": "Complete guide to building ORMs in Rust",
                "published": true,
                "view_count": 150,
                "user_id": 1
            },
            {
                "id": 2,
                "title": "Axum Integration",
                "content": "How to integrate RustORM with Axum",
                "published": true,
                "view_count": 89,
                "user_id": 1
            }
        ]
    },
    {
        "id": 2,
        "name": "Sara Ahmed",
        "email": "sara@example.com",
        "place": "Karachi",
        "age": 25,
        "bio": "Backend engineer",
        "posts": [
            {
                "id": 3,
                "title": "Database Design Patterns",
                "content": "Best practices for relational databases",
                "published": false,
                "view_count": 42,
                "user_id": 2
            }
        ]
    }
]
```

### GET /users/with-posts-comments
**Expected Response:**
```json
[
    {
        "id": 1,
        "name": "Ali Khan",
        "email": "ali@example.com",
        "place": "Lahore",
        "age": 28,
        "bio": "Full-stack developer",
        "posts": [
            {
                "id": 1,
                "title": "Rust ORM Guide",
                "content": "Complete guide to building ORMs in Rust",
                "published": true,
                "view_count": 150,
                "user_id": 1,
                "comments": []
            },
            {
                "id": 2,
                "title": "Axum Integration",
                "content": "How to integrate RustORM with Axum",
                "published": true,
                "view_count": 89,
                "user_id": 1,
                "comments": []
            }
        ]
    }
]
```

> 💡 Comments empty hain kyunke abhi seed nahi kiye. Agar comments bhi test karne hain toh pehle comments table mein data daalo.

### GET /users/with-profile
**Expected Response:**
```json
[
    {
        "id": 1,
        "name": "Ali Khan",
        "email": "ali@example.com",
        "place": "Lahore",
        "age": 28,
        "bio": "Full-stack developer",
        "profile": null
    },
    {
        "id": 2,
        "name": "Sara Ahmed",
        "email": "sara@example.com",
        "place": "Karachi",
        "age": 25,
        "bio": "Backend engineer",
        "profile": null
    }
]
```

> 💡 Profile null hai kyunke user_profiles table mein data nahi hai.

### GET /posts/with-user
**Expected Response:**
```json
[
    {
        "id": 1,
        "title": "Rust ORM Guide",
        "content": "Complete guide to building ORMs in Rust",
        "published": true,
        "view_count": 150,
        "user_id": 1,
        "user": {
            "id": 1,
            "name": "Ali Khan",
            "email": "ali@example.com",
            "place": "Lahore",
            "age": 28,
            "bio": "Full-stack developer"
        }
    },
    {
        "id": 2,
        "title": "Axum Integration",
        "content": "How to integrate RustORM with Axum",
        "published": true,
        "view_count": 89,
        "user_id": 1,
        "user": {
            "id": 1,
            "name": "Ali Khan",
            "email": "ali@example.com",
            "place": "Lahore",
            "age": 28,
            "bio": "Full-stack developer"
        }
    },
    {
        "id": 3,
        "title": "Database Design Patterns",
        "content": "Best practices for relational databases",
        "published": false,
        "view_count": 42,
        "user_id": 2,
        "user": {
            "id": 2,
            "name": "Sara Ahmed",
            "email": "sara@example.com",
            "place": "Karachi",
            "age": 25,
            "bio": "Backend engineer"
        }
    }
]
```

## Verification Checklist

| Check | Endpoint | Kya Dekhna Hai |
|---|---|---|
| ✅ Basic CRUD | `GET /users` | Users list aa rahi hai |
| ✅ Single-level N:1 | `GET /posts/with-user` | Har post mein `user` object populated |
| ✅ Single-level 1:N | `GET /users/with-posts` | Har user mein `posts` array populated |
| ✅ Single-level 1:1 | `GET /users/with-profile` | `profile` field present (null ya object) |
| ✅ **Nested 2-level** | `GET /users/with-posts-comments` | User → posts → comments teeno levels populated |
| ✅ No crash | All endpoints | 500 error nahi aa raha |

Pehle seed data banao, phir eager loading endpoints test karo aur actual responses bhejte jao! 🚀