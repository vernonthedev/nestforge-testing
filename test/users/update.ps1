<#
--------------------------------------
    Updating a user using user_id
--------------------------------------
#>
$user_id = 4
$update_user_uri = "http://localhost:3000/api/v1/users/$user_id"
$update_body = @{
    name="John"
} | ConvertTo-Json

Invoke-RestMethod -Uri $update_user_uri -Method Put -ContentType "application/json" -Body $update_body