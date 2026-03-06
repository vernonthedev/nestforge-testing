<#
--------------------------------------
    Creating a new user
--------------------------------------
#>
Write-Host "[+] Create A New User.." -ForegroundColor Cyan -BackgroundColor DarkGreen
$create_user_uri = "http://localhost:3000/api/v2/users"
$create_user_body = @{
    name="vernonthedev"
} | ConvertTo-Json

Invoke-RestMethod -Uri $create_user_uri -Method Post -ContentType "application/json" -Body $create_user_body
