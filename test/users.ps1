# Users API request Testing

# Creating a new user
$uri = "http://localhost:3000/api/v1/users"
$body = @{
    name="samuel"
} | ConvertTo-Json

Invoke-RestMethod -Uri $uri -Method Post -ContentType "application/json" -Body $body