
<#
                --------------------------------------
                    Users API request Testing
                --------------------------------------
#>

# CREATE USER
# 
# $create_user_uri = "http://localhost:3000/api/v1/users"
# $create_user_body = @{
#     name="samuel"
# } | ConvertTo-Json

# Invoke-RestMethod -Uri $create_user_uri -Method Post -ContentType "application/json" -Body $create_user_body

# GET USERS
$get_users_uri = "http://localhost:3000/api/v1/users"
Invoke-RestMethod -Uri $get_users_uri -Method Get

# GET USER
$id = 4
$get_user_uri = "http://localhost:3000/api/v1/users/$id"
Invoke-RestMethod -Uri $get_user_uri -Method Get

# UPDATE USER
$user_id = 4
$update_user_uri = "http://localhost:3000/api/v1/users/$user_id"
$update_body = @{
    name="John"
} | ConvertTo-Json

Invoke-RestMethod -Uri $update_user_uri -Method Put -ContentType "application/json" -Body $update_body

# GET USER
$id = 4
$get_user_uri = "http://localhost:3000/api/v1/users/$id"
Invoke-RestMethod -Uri $get_user_uri -Method Get

# DELETE USER