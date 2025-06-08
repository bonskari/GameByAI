#!/usr/bin/env pwsh
# Visual Testing Script for Wolfenstein by AI
# Runs automated visual tests with bot movement

Write-Host "🤖 Wolfenstein by AI - Visual Testing Suite" -ForegroundColor Cyan
Write-Host "=============================================" -ForegroundColor Cyan

# Test configurations
$tests = @(
    @{
        Name = "Quick Visual Test"
        Duration = 10
        AutoClose = $true
        Description = "Fast 10-second test with auto-close"
    },
    @{
        Name = "Standard Visual Test" 
        Duration = 15
        AutoClose = $true
        Description = "Standard 15-second test with auto-close"
    },
    @{
        Name = "Extended Visual Test"
        Duration = 30
        AutoClose = $true
        Description = "Extended 30-second test with auto-close"
    }
)

# Function to run a visual test
function Run-VisualTest {
    param(
        [string]$TestName,
        [int]$Duration,
        [bool]$AutoClose,
        [string]$Description
    )
    
    Write-Host "`n🚀 Running: $TestName" -ForegroundColor Green
    Write-Host "   $Description" -ForegroundColor Gray
    Write-Host "   Duration: ${Duration}s, Auto-close: $AutoClose" -ForegroundColor Gray
    
    $startTime = Get-Date
    
    if ($AutoClose) {
        & "target/release/wolfenstein-ai.exe" visual-test --duration $Duration
    } else {
        & "target/release/wolfenstein-ai.exe" visual-test --duration $Duration --no-auto-close
    }
    
    $endTime = Get-Date
    $actualDuration = ($endTime - $startTime).TotalSeconds
    
    Write-Host "   ✅ Completed in $([math]::Round($actualDuration, 1))s" -ForegroundColor Green
}

# Check if executable exists
if (-not (Test-Path "target/release/wolfenstein-ai.exe")) {
    Write-Host "❌ Error: Game executable not found!" -ForegroundColor Red
    Write-Host "   Please run 'cargo build --release' first" -ForegroundColor Yellow
    exit 1
}

# Show menu
Write-Host "`nAvailable Visual Tests:" -ForegroundColor Yellow
for ($i = 0; $i -lt $tests.Count; $i++) {
    $test = $tests[$i]
    Write-Host "  $($i + 1). $($test.Name) - $($test.Description)" -ForegroundColor White
}
Write-Host "  A. Run All Tests" -ForegroundColor White
Write-Host "  Q. Quit" -ForegroundColor White

# Get user choice
do {
    $choice = Read-Host "`nSelect test to run (1-$($tests.Count), A, Q)"
    
    switch ($choice.ToUpper()) {
        "Q" { 
            Write-Host "Goodbye! 👋" -ForegroundColor Cyan
            exit 0 
        }
        "A" {
            Write-Host "`n🎯 Running All Visual Tests..." -ForegroundColor Magenta
            foreach ($test in $tests) {
                Run-VisualTest -TestName $test.Name -Duration $test.Duration -AutoClose $test.AutoClose -Description $test.Description
                Start-Sleep -Seconds 2  # Brief pause between tests
            }
            Write-Host "`n🎉 All visual tests completed!" -ForegroundColor Green
            break
        }
        default {
            $testIndex = [int]$choice - 1
            if ($testIndex -ge 0 -and $testIndex -lt $tests.Count) {
                $test = $tests[$testIndex]
                Run-VisualTest -TestName $test.Name -Duration $test.Duration -AutoClose $test.AutoClose -Description $test.Description
                break
            } else {
                Write-Host "Invalid choice. Please try again." -ForegroundColor Red
            }
        }
    }
} while ($true)

Write-Host "`n🏁 Visual testing session complete!" -ForegroundColor Cyan 