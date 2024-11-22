import { useMutation } from '@tanstack/react-query';
import jammy from '../lib/jammy';

export default function useLogin() {
    async function login({ email, pass }: { email: string, pass: string }) {
        console.log('Logging in with:', email, pass);

        const resp = await fetch('/api/user/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username: email, password: pass }),
        });

        if (!resp.ok) {
            throw new Error('Login failed');
        }

        const data = await resp.text();
        console.log('Login token:', data);

        jammy.token = data;
    }

    return useMutation({
        mutationFn: login,
        onSuccess: (_, __, ___) => {
            jammy.isValid = true;
        },
    });
}
