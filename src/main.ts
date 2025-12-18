import { invoke } from '@tauri-apps/api/core';

const toggleBtn = document.getElementById('toggle-btn') as HTMLButtonElement;
const btnText = document.getElementById('btn-text') as HTMLSpanElement;
const statusText = document.getElementById('status-text') as HTMLParagraphElement;

let isActive = false;

// Update status UI
function updateStatusUI() {
  if (isActive) {
    statusText.textContent = 'Active';
    statusText.classList.remove('inactive');
    statusText.classList.add('active');
    btnText.textContent = 'Disable';
  } else {
    statusText.textContent = 'Inactive';
    statusText.classList.remove('active');
    statusText.classList.add('inactive');
    btnText.textContent = 'Enable';
  }
}

// Check initial state from Rust
async function checkInitialState() {
  try {
    isActive = await invoke<boolean>('is_playing');
    updateStatusUI();
  } catch (err) {
    console.error('Failed to get initial audio state:', err);
  }
}

// Toggle button click handler
toggleBtn.addEventListener('click', async () => {
  try {
    if (!isActive) {
      await invoke('start_audio');
      isActive = true;
    } else {
      await invoke('stop_audio');
      isActive = false;
    }
    updateStatusUI();
  } catch (err) {
    console.error('Failed to toggle audio:', err);
  }
});

// Initialize
checkInitialState();
