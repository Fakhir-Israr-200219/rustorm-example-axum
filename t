# Create user
curl -X POST http://localhost:3000/users -H "Content-Type: application/json" -d "{\"name\":\"Fakhir\",\"email\":\"fakhir@test.com\"}"

# List users
curl http://localhost:3000/users

# Create post
curl -X POST http://localhost:3000/posts -H "Content-Type: application/json" -d "{\"title\":\"First Post\",\"content\":\"Hello World\",\"user_id\":1}"

# List posts
curl http://localhost:3000/posts