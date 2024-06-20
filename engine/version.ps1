param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

npm version $Version;
pnpm -r exec npm version $Version;
