function smake.install()
    run('npm i -g pnpm --silent')
    runIn('./crates/frontend', 'pnpm i')
end

function smake.run() 
    runIn('./crates/backend', 'cargo tauri dev')
end

function smake.build()
    runIn('./crates/backend', 'cargo tauri build')
end

function smake.addPackage(package, flag) 
    local isBackend = flag == '-b'
    runIn('./crates/' .. (isBackend and 'backend' or 'frontend'), (isBackend and 'cargo add ' or 'pnpm i ') .. package)
end

function smake.removePackage(package, flag) 
    local isBackend = flag == '-b'
    runIn('./crates/' .. (isBackend and 'backend' or 'frontend'), (isBackend and 'cargo remove ' or 'pnpm uninstall ') .. package)
end
