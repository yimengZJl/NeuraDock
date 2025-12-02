import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { NotificationChannelDto } from '@/components/notification/NotificationChannelList';

export function useNotificationChannels() {
  return useQuery({
    queryKey: ['notification-channels'],
    queryFn: async () => {
      const channels = await invoke<NotificationChannelDto[]>('get_all_notification_channels');
      return channels;
    },
    refetchInterval: false,
  });
}
