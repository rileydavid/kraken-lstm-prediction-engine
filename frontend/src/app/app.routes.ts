import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DashboardComponent } from './dashboard/dashboard.component';
import { ConfigComponent } from './config/config.component';

export const routes: Routes = [
    { path: 'dashboard', component: DashboardComponent },
    { path: 'config', component: ConfigComponent },
    { path: '', redirectTo: '/dashboard', pathMatch: 'full' },
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})
export class AppRoutingModule { }