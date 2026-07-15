import { createTheme } from '@mui/material/styles';

// StellarEscrow brand palette
// primary   #1a1a2e  — deep navy
// secondary #e94560  — coral red (actions, alerts)
// success   #2ecc71  — green (funded, completed states)
const theme = createTheme({
  palette: {
    primary: { main: '#1a1a2e' },
    secondary: { main: '#e94560' },
    success: { main: '#2ecc71' },
    background: { default: '#f5f5f5' },
  },
  typography: {
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
  },
  components: {
    MuiButton: {
      defaultProps: { disableElevation: true },
      styleOverrides: { root: { textTransform: 'none', borderRadius: 6 } },
    },
  },
});

export default theme;
