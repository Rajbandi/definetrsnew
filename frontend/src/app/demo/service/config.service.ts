import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';
import { LoggingService } from './logging.service';

@Injectable({
  providedIn: 'root'
})
export class ConfigService {
  private configUrl = 'assets/main/config.json';

  constructor(private http: HttpClient, private logging: LoggingService) {}

  async getConfig(): Promise<any> {
    this.logging.info('Loading config...');
    return await firstValueFrom(this.http.get(this.configUrl));
  }
}
