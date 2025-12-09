// Bot API Client for Curverider Vault
// Communicates with the Rust bot's HTTP API

const BOT_API_URL = process.env.NEXT_PUBLIC_BOT_API_URL || 'http://localhost:8080';

export interface Strategy {
  id: string;
  name: string;
  description: string;
  risk_level: string;
  target_return: string;
  win_rate: string;
  hold_time: string;
}

export interface Position {
  position_id: string;
  user: string;
  token_mint: string;
  token_symbol: string;
  amount_sol: number;
  entry_price: number;
  current_price: number;
  take_profit_price: number;
  stop_loss_price: number;
  status: string;
  pnl: number;
  pnl_percentage: number;
  opened_at: number;
  closed_at?: number;
}

export interface UserStats {
  wallet: string;
  strategy: string;
  is_active: boolean;
  active_positions: number;
  total_trades: number;
  profitable_trades: number;
  win_rate: number;
  total_pnl_sol: number;
  total_pnl_usd: number;
}

export interface BotHealth {
  status: string;
  version: string;
  uptime_seconds: number;
}

export interface BotStats {
  is_running: boolean;
  uptime_seconds: number;
  total_scans: number;
  tokens_analyzed: number;
  signals_generated: number;
  trades_executed: number;
  last_scan_at?: number;
}

class BotApiClient {
  private baseUrl: string;

  constructor(baseUrl: string = BOT_API_URL) {
    this.baseUrl = baseUrl;
  }

  private async fetch<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`);
    if (!response.ok) {
      throw new Error(`API request failed: ${response.statusText}`);
    }
    return response.json();
  }

  async getHealth(): Promise<BotHealth> {
    return this.fetch<BotHealth>('/api/health');
  }

  async getStrategies(): Promise<Strategy[]> {
    return this.fetch<Strategy[]>('/api/strategies');
  }

  async getUserPositions(wallet: string): Promise<Position[]> {
    return this.fetch<Position[]>(`/api/users/${wallet}/positions`);
  }

  async getUserStats(wallet: string): Promise<UserStats> {
    return this.fetch<UserStats>(`/api/users/${wallet}/stats`);
  }

  async getAllPositions(): Promise<Position[]> {
    return this.fetch<Position[]>('/api/positions');
  }

  async getBotStats(): Promise<BotStats> {
    return this.fetch<BotStats>('/api/stats');
  }

  // WebSocket connection for real-time updates
  connectWebSocket(onMessage: (data: BotStats) => void): WebSocket {
    const wsUrl = this.baseUrl.replace('http', 'ws');
    const ws = new WebSocket(`${wsUrl}/api/stream`);

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        onMessage(data);
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    ws.onclose = () => {
      console.log('WebSocket connection closed');
    };

    return ws;
  }
}

export const botApi = new BotApiClient();
