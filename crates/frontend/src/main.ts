import './style.css';
import App from './App.svelte';

import { appWindow } from '@tauri-apps/api/window';

(async () => {
    // get and set values
    document.addEventListener('keydown', (event) => {
        if (event.key === 'Escape') {
            appWindow.hide();
        }
    });
})();

const app = new App({
    target: document.getElementById('app'),
});

export default app;
