<#
--------------------------------------
    Updating a user using user_id
--------------------------------------
#>
Write-Host "[+] Updating A User.." -ForegroundColor Cyan -BackgroundColor DarkGreen
$user_id = 1
$update_user_uri = "http://localhost:3000/api/v2/users/$user_id"
$update_body = @{
    name="mos demrand"
} | ConvertTo-Json

Invoke-RestMethod -Uri $update_user_uri -Method Put -ContentType "application/json" -Body $update_body