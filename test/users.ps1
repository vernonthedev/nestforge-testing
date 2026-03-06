
<#
--------------------------------------
Users API request Testing
--------------------------------------
#>

# CREATE USER
$id = 4
$uri = "http://localhost:3000/api/v1/users/$id"
$body = @{
    name="samuel"
} | ConvertTo-Json

# Invoke-RestMethod -Uri $uri -Method Post -ContentType "application/json" -Body $body

# GET USERS
Invoke-RestMethod -Uri $uri -Method Get

