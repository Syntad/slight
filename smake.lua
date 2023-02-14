function smake.install()
    run('npm i -g yarn --silent')
    runIn('./crates/frontend', 'yarn')
end

function smake.run() 
    runIn('./crates/backend', 'cargo tauri dev')
end

function smake.build()
    runIn('./crates/backend', 'cargo tauri build')
end

function smake.addPackage(package, flag, mode) 
    local isBackend = flag == '-b'
    runIn('./crates/' .. (isBackend and 'backend' or 'frontend'), (isBackend and 'cargo add ' or 'yarn add ' .. (mode .. ' ' or '')) .. package)
end

function smake.removePackage(package, flag, mode) 
    local isBackend = flag == '-b'
    runIn('./crates/' .. (isBackend and 'backend' or 'frontend'), (isBackend and 'cargo remove ' or 'yarn remove ') .. package)
end
