import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { TokenInfo } from 'app/models/token-info';
import { Observable } from 'rxjs';
import { environment } from '../../environements/environment'; // Import environment

@Injectable({
  providedIn: 'root'
})
export class TokenService {

    private apiUrl = `${environment.backendUrl}/get-all-tokens`; // Use environment variable

    constructor(private http: HttpClient) {}

    getAllTokens(): Observable<TokenInfo[]> {
      return this.http.get<TokenInfo[]>(this.apiUrl);
    }
}
