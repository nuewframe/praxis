export interface UserPort {
  getUser(id: string): Promise<User>;
  saveUser(user: User): Promise<boolean>;
}
