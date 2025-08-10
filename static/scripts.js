// scripts.js

'use strict';

// Utility to attach handlers after DOM is ready
function onReady(fn) {
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', fn, { once: true });
  } else {
    fn();
  }
}

onReady(() => {
  // Login form handler
  const loginForm = document.getElementById('login-form');
  if (loginForm) {
    loginForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      const form = e.currentTarget;
      const payload = {
        username_or_email: form.username_or_email.value,
        password: form.password.value,
      };
      try {
        const r = await fetch('/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload),
        });
        if (r.ok) {
          window.location.assign('/admin');
        } else {
          const t = await r.text();
          alert('Login failed' + (t ? (': ' + t) : ''));
          // Clear the form after the user dismisses the alert
          form.reset();
          const first = form.querySelector('input[name="username_or_email"]');
          if (first) first.focus();
        }
      } catch (err) {
        alert('Login failed: network error');
        // Clear the form after the user dismisses the alert
        form.reset();
        const first = form.querySelector('input[name="username_or_email"]');
        if (first) first.focus();
      }
    });

    // Show/hide password toggle
    const toggle = document.getElementById('toggle-password');
    const pwd = loginForm.querySelector('input[name="password"]');
    if (toggle && pwd) {
      toggle.addEventListener('change', () => {
        pwd.type = toggle.checked ? 'text' : 'password';
      });
    }
  }

  // Registration form handler
  const registerForm = document.getElementById('register-form');
  if (registerForm) {
    registerForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      const form = e.currentTarget;
      const payload = {
        username: form.username.value,
        email: form.email.value,
        password: form.password.value,
        display_name: form.display_name.value || null,
      };
      try {
        const r = await fetch('/auth/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload),
        });
        if (r.ok) {
          window.location.assign('/login');
        } else {
          const t = await r.text();
          alert('Registration failed' + (t ? (': ' + t) : ''));
        }
      } catch (err) {
        alert('Registration failed: network error');
      }
    });

    // Show/hide password toggle for registration
    const toggleReg = document.getElementById('toggle-password-register');
    const pwdReg = registerForm.querySelector('input[name="password"]');
    if (toggleReg && pwdReg) {
      toggleReg.addEventListener('change', () => {
        pwdReg.type = toggleReg.checked ? 'text' : 'password';
      });
    }
  }

  // Logout button handler
  const logoutBtn = document.getElementById('logout-btn');
  if (logoutBtn) {
    logoutBtn.addEventListener('click', async (e) => {
      e.preventDefault();
      try {
        const r = await fetch('/auth/logout', { method: 'POST' });
        if (r.ok) {
          window.location.assign('/');
        } else {
          const t = await r.text();
          alert('Logout failed' + (t ? (': ' + t) : ''));
        }
      } catch (err) {
        alert('Logout failed: network error');
      }
    });
  }

  // Inject current year into footer span#year
  const yearSpan = document.getElementById('year');
  if (yearSpan) {
    yearSpan.textContent = String(new Date().getFullYear());
  }
});
