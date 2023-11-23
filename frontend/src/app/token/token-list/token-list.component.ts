import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TokenInfo } from 'app/models/token-info';
import { TokenService } from 'app/services/token.service';

@Component({
  selector: 'app-token-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './token-list.component.html',
  styleUrl: './token-list.component.scss'
})
export class TokenListComponent {
    tokens: TokenInfo[] = [];
    constructor(private tokenService: TokenService) {}

    ngOnInit(): void {
      this.tokenService.getAllTokens().subscribe(data => {
        this.tokens = data;
      });
    }
}
