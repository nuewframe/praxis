export class UserMemoryAdapter {
  getUser(id: string): Promise<User> {
    return Promise.resolve({ id });
  }
  saveUser(user: User): Promise<boolean> {
    return Promise.resolve(true);
  }
}
