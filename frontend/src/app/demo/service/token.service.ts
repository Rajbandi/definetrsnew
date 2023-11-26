import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject, Observable, Subject, firstValueFrom } from 'rxjs';
import { TokenInfo } from '../models/token-info';
import { ConfigService } from './config.service';
import { LoggingService } from './logging.service';
import { cu } from '@fullcalendar/core/internal-common';

@Injectable({
  providedIn: 'root'
})
export class TokenService {
    private ws: WebSocket;
    private tokensSubject = new BehaviorSubject<TokenInfo[]>([]);
    public tokens$: Observable<TokenInfo[]> = this.tokensSubject.asObservable();

    private apiUrl = '/api'; // Proxy path
    private readonly wsUrl = 'ws://localhost:4200/ws/updates'; // WebSocket URL using proxy

  constructor(private http: HttpClient, private logging: LoggingService) {
    this.connectToWebSocket();
  }

  async loadApiUrl(): Promise<void> {

    this.logging.info('API URL:', this.apiUrl);

  }
  async fetchTokens(): Promise<void> {

    const tokens = await this.getTokens();
    this.tokensSubject.next(tokens);
  }

  async getTokens(): Promise<TokenInfo[]> {
    this.logging.info('Loading tokens...');
    let tokens_url = this.apiUrl +'/get_all_tokens?sort_by=date_created desc&limit=50';
    return await firstValueFrom(this.http.get<TokenInfo[]>(tokens_url));
  }
  private connectToWebSocket(): void {
    this.logging.info('Connecting to WebSocket...');
    this.ws = new WebSocket(this.wsUrl);
    this.logging.info('WebSocket state:', this.ws.readyState);

    this.ws.onmessage = (event) => {
      this.logging.info('WebSocket message received:');
      const updatedToken: TokenInfo = JSON.parse(event.data);
      this.handleTokenUpdate(updatedToken);
    };

    this.ws.onerror = (error) => {
        this.logging.error('WebSocket error:', error);
      console.error('WebSocket error:', error);
    };

    this.ws.onclose = (event) => {
      console.log('WebSocket closed. Attempting to reconnect...', event);
      setTimeout(() => this.connectToWebSocket(), 3000); // Reconnect logic
    };
  }
  private handleTokenUpdate(socketData: any): void {
    let updatedToken = socketData.data as TokenInfo;
    if(!updatedToken || !updatedToken.contractAddress || !updatedToken.symbol || !updatedToken.name){
        this.logging.error('Invalid token:', updatedToken);
        return;
    }
    this.logging.info('Updating token:', updatedToken.contractAddress);
    const currentTokens = this.tokensSubject.getValue();

    const index = currentTokens.findIndex(t => t.contractAddress === updatedToken.contractAddress);
    if (index > -1) {
      currentTokens[index] = updatedToken;
    } else {
        const newTokenWithHighlight = { ...updatedToken, isNew: true };
      currentTokens.unshift(newTokenWithHighlight);
      
    }
    this.tokensSubject.next([...currentTokens]); // Emit a new array to trigger change detection
  }
  // Send a message through the WebSocket
  sendWebSocketMessage(message: any): void {
    if (this.ws.readyState === WebSocket.OPEN) {
        this.logging.info('Sending WebSocket message:', message);
      this.ws.send(JSON.stringify(message));
    }
  }

}
