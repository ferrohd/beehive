import { createContext, useState, useContext, useEffect, ReactElement } from 'react';

const ThemeContext = createContext<{ theme: string, toggleTheme: Function }>({
    theme: 'light',
    toggleTheme: (theme: string) => null
});

export const useTheme = () => useContext(ThemeContext);

export const ThemeProvider = ({ children }: { children: ReactElement }) => {
    // Initialize theme based on system preference
    const [theme, setTheme] = useState(() => {
        if (typeof window !== 'undefined') {
            return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
        }
        return 'light';
    });

    const toggleTheme = (theme: string) => {
        setTheme(theme);
    };

    useEffect(() => {
        const handleChange = (e: MediaQueryListEvent) => {
            setTheme(e.matches ? 'dark' : 'light');
        };

        // Listen for system theme changes
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
        mediaQuery.addEventListener('change', handleChange);

        // Update document classes
        document.documentElement.classList.remove('light', 'dark');
        document.documentElement.classList.add(theme);

        // Cleanup listener
        return () => mediaQuery.removeEventListener('change', handleChange);
    }, [theme]);

    return (
        <ThemeContext.Provider value={{ theme, toggleTheme }}>
            {children}
        </ThemeContext.Provider>
    );
};
